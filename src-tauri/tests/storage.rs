use rojekti_lib::storage::{parse_card_file, serialize_card_file, ensure_ids, detect_renames};
use rojekti_lib::models::{CardMeta, BoardConfig, Epic, Tag, Status};

#[test]
fn test_detect_renames_reorder_only() {
    let mut old_config = BoardConfig {
        name: "Test".into(),
        prefix: "TEST".into(),
        next_id: 1,
        statuses: vec![],
        epics: vec![
            Epic { id: "id-1".into(), name: "Alpha".into(), color: "#000".into(), pending_rename: None },
            Epic { id: "id-2".into(), name: "Beta".into(), color: "#000".into(), pending_rename: None },
        ],
        tags: vec![],
        priorities: vec![],
    };

    // New config has the same epics but in reverse order
    let new_config = BoardConfig {
        name: "Test".into(),
        prefix: "TEST".into(),
        next_id: 1,
        statuses: vec![],
        epics: vec![
            Epic { id: "id-2".into(), name: "Beta".into(), color: "#000".into(), pending_rename: None },
            Epic { id: "id-1".into(), name: "Alpha".into(), color: "#000".into(), pending_rename: None },
        ],
        tags: vec![],
        priorities: vec![],
    };

    let (epic_renames, status_renames) = detect_renames(&mut old_config, &new_config);

    assert!(!epic_renames, "Reordering should not trigger epic renames");
    assert!(!status_renames, "Reordering should not trigger status renames");
    assert!(old_config.epics[0].pending_rename.is_none());
    assert!(old_config.epics[1].pending_rename.is_none());
}

#[test]
fn test_detect_renames_actual_rename() {
    let mut old_config = BoardConfig {
        name: "Test".into(),
        prefix: "TEST".into(),
        next_id: 1,
        statuses: vec![],
        epics: vec![
            Epic { id: "id-1".into(), name: "Old Name".into(), color: "#000".into(), pending_rename: None },
        ],
        tags: vec![],
        priorities: vec![],
    };

    let new_config = BoardConfig {
        name: "Test".into(),
        prefix: "TEST".into(),
        next_id: 1,
        statuses: vec![],
        epics: vec![
            Epic { id: "id-1".into(), name: "New Name".into(), color: "#000".into(), pending_rename: None },
        ],
        tags: vec![],
        priorities: vec![],
    };

    let (epic_renames, _) = detect_renames(&mut old_config, &new_config);

    assert!(epic_renames);
    assert_eq!(old_config.epics[0].pending_rename, Some("New Name".into()));
}

#[test]
fn test_parse_card_file_valid() {
    let content = "---\nid: roj-001\ntitle: Test Card\nstatus: backlog\nepic: Board\ntags: [bug]\npriority: 3\nposition: 1.0\ncreated: 2026-03-23\n---\n\nBody content here";
    let (meta, body) = parse_card_file(content).unwrap();
    
    assert_eq!(meta.id, "roj-001");
    assert_eq!(meta.title, "Test Card");
    assert_eq!(meta.status, "backlog");
    assert_eq!(meta.epic, Some("Board".to_string()));
    assert_eq!(meta.tags, vec!["bug".to_string()]);
    assert_eq!(meta.priority, 3);
    assert_eq!(meta.position, 1.0);
    assert_eq!(meta.created, "2026-03-23");
    assert_eq!(body, "Body content here");
}

#[test]
fn test_parse_card_file_missing_delimiters() {
    let result = parse_card_file("no frontmatter here");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("missing '---' delimiters"));
}

#[test]
fn test_parse_card_file_malformed_yaml() {
    let content = "---\nid: roj-001\ntitle: : malformed\n---\nbody";
    let result = parse_card_file(content);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("YAML parse error"));
}

#[test]
fn test_parse_card_file_no_body() {
    let content = "---\nid: roj-001\ntitle: Test\nstatus: todo\ntags: []\npriority: 1\nposition: 1.0\ncreated: 2026-03-23\n---";
    let (meta, body) = parse_card_file(content).unwrap();
    assert_eq!(meta.id, "roj-001");
    assert_eq!(body, "");
}

#[test]
fn test_card_file_round_trip() {
    let meta = CardMeta {
        id: "roj-001".into(),
        title: "Test".into(),
        status: "todo".into(),
        epic: None,
        tags: vec!["tag1".into()],
        priority: 2,
        position: 1.5,
        created: "2026-03-23".into(),
    };
    let original_body = "Line 1\nLine 2";
    let serialized = serialize_card_file(&meta, original_body);
    let (parsed_meta, parsed_body) = parse_card_file(&serialized).unwrap();
    
    assert_eq!(parsed_meta.id, meta.id);
    assert_eq!(parsed_meta.tags, meta.tags);
    assert_eq!(parsed_meta.priority, meta.priority);
    assert_eq!(parsed_body, original_body);
}

#[test]
fn test_ensure_ids_none_missing() {
    let mut config = BoardConfig {
        name: "Test".into(),
        prefix: "TEST".into(),
        next_id: 1,
        statuses: vec![Status { id: "s1".into(), name: "S1".into(), pending_rename: None }],
        epics: vec![Epic { id: "e1".into(), name: "E1".into(), color: "#000".into(), pending_rename: None }],
        tags: vec![Tag { id: "t1".into(), name: "T1".into(), color: "#000".into(), pending_rename: None }],
        priorities: vec![],
    };
    
    let changed = ensure_ids(&mut config);
    assert!(!changed);
    assert_eq!(config.statuses[0].id, "s1");
    assert_eq!(config.epics[0].id, "e1");
    assert_eq!(config.tags[0].id, "t1");
}

#[test]
fn test_ensure_ids_all_missing() {
    let mut config = BoardConfig {
        name: "Test".into(),
        prefix: "TEST".into(),
        next_id: 1,
        statuses: vec![Status { id: "".into(), name: "S1".into(), pending_rename: None }],
        epics: vec![Epic { id: "".into(), name: "E1".into(), color: "#000".into(), pending_rename: None }],
        tags: vec![Tag { id: "".into(), name: "T1".into(), color: "#000".into(), pending_rename: None }],
        priorities: vec![],
    };
    
    let changed = ensure_ids(&mut config);
    assert!(changed);
    assert!(!config.statuses[0].id.is_empty());
    assert!(!config.epics[0].id.is_empty());
    assert!(!config.tags[0].id.is_empty());
    // Verify they are valid UUIDs (contain hyphens)
    assert!(config.statuses[0].id.contains("-"));
}

#[test]
fn test_ensure_ids_mixed() {
    let mut config = BoardConfig {
        name: "Test".into(),
        prefix: "TEST".into(),
        next_id: 1,
        statuses: vec![
            Status { id: "s1".into(), name: "S1".into(), pending_rename: None },
            Status { id: "".into(), name: "S2".into(), pending_rename: None }
        ],
        epics: vec![],
        tags: vec![],
        priorities: vec![],
    };
    
    let changed = ensure_ids(&mut config);
    assert!(changed);
    assert_eq!(config.statuses[0].id, "s1");
    assert!(!config.statuses[1].id.is_empty());
}

#[test]
fn test_board_config_backward_compatibility() {
    // YAML representing an old config without ID fields
    let legacy_yaml = r##"
name: Legacy Board
prefix: ROJ
nextId: 10
statuses:
  - name: Todo
epics:
  - name: Board
    color: "#000"
tags:
  - name: Bug
    color: "#ff0000"
priorities: []
"##;
    let config: BoardConfig = serde_yaml::from_str(legacy_yaml).unwrap();
    
    assert_eq!(config.name, "Legacy Board");
    assert_eq!(config.statuses[0].name, "Todo");
    assert_eq!(config.statuses[0].id, "", "Missing ID should default to empty string");
    assert_eq!(config.epics[0].id, "");
    assert_eq!(config.tags[0].id, "");
}

#[test]
fn test_board_config_round_trip() {
    let config = BoardConfig {
        name: "Full Config".into(),
        prefix: "TEST".into(),
        next_id: 5,
        statuses: vec![Status { id: "s-1".into(), name: "S1".into(), pending_rename: Some("New S1".into()) }],
        epics: vec![Epic { id: "e-1".into(), name: "E1".into(), color: "#fff".into(), pending_rename: None }],
        tags: vec![Tag { id: "t-1".into(), name: "T1".into(), color: "#abc".into(), pending_rename: None }],
        priorities: vec![],
    };
    
    let serialized = serde_yaml::to_string(&config).unwrap();
    let deserialized: BoardConfig = serde_yaml::from_str(&serialized).unwrap();
    
    assert_eq!(deserialized.name, config.name);
    assert_eq!(deserialized.statuses[0].id, "s-1");
    assert_eq!(deserialized.statuses[0].pending_rename, Some("New S1".into()));
    assert_eq!(deserialized.epics[0].color, "#fff");
}
