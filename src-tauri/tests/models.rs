use rojekti_lib::models::CardMeta;

#[test]
fn test_deserialize_priority_number() {
    let yaml = "id: r-1\ntitle: T\nstatus: s\npriority: 3\nposition: 1\ncreated: c";
    let meta: CardMeta = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(meta.priority, 3);
}

#[test]
fn test_deserialize_priority_string() {
    let yaml = "id: r-1\ntitle: T\nstatus: s\npriority: \"4\"\nposition: 1\ncreated: c";
    let meta: CardMeta = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(meta.priority, 4);
}

#[test]
fn test_deserialize_priority_out_of_range() {
    let yaml = "id: r-1\ntitle: T\nstatus: s\npriority: 6\nposition: 1\ncreated: c";
    let meta: CardMeta = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(meta.priority, 0);
}

#[test]
fn test_deserialize_priority_invalid_type() {
    let yaml = "id: r-1\ntitle: T\nstatus: s\npriority: null\nposition: 1\ncreated: c";
    let meta: CardMeta = serde_yaml::from_str(yaml).unwrap();
    assert_eq!(meta.priority, 0);
}
