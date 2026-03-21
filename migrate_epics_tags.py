#!/usr/bin/env python3
"""
One-time migration: replace epic/tag IDs with names in all card files and config.
Run from the project root: python3 migrate_epics_tags.py
"""

import os
import re

CARDS_DIR = "rojekti/cards"
CONFIG_PATH = "rojekti/rojekti.config.yaml"


def read_file(path):
    with open(path, "r", encoding="utf-8") as f:
        return f.read()


def write_file(path, content):
    tmp = path + ".tmp"
    with open(tmp, "w", encoding="utf-8") as f:
        f.write(content)
    os.replace(tmp, path)


def parse_id_name_map(config_content, section):
    """Extract id → name mapping from a YAML section (epics or tags)."""
    result = {}
    in_section = False
    current_id = None

    for line in config_content.splitlines():
        if re.match(rf"^{section}:", line):
            in_section = True
            continue
        if in_section:
            if line and not line.startswith(" ") and not line.startswith("-"):
                break  # left the section
            id_match = re.match(r"^-?\s*id:\s*['\"]?(.+?)['\"]?\s*$", line)
            name_match = re.match(r"^\s+name:\s*['\"]?(.+?)['\"]?\s*$", line)
            if id_match:
                current_id = id_match.group(1).strip()
            if name_match and current_id:
                result[current_id] = name_match.group(1).strip()
                current_id = None

    return result


def migrate_card(path, epic_map, tag_map):
    content = read_file(path)
    parts = content.split("---", 2)
    if len(parts) < 3:
        return False

    frontmatter = parts[1]
    body = parts[2]
    changed = False

    # Replace epic ID
    def replace_epic(m):
        nonlocal changed
        val = m.group(1).strip().strip("'\"")
        if val in epic_map:
            changed = True
            return f"epic: {epic_map[val]}"
        return m.group(0)

    frontmatter = re.sub(r"^epic:\s*(.+)$", replace_epic, frontmatter, flags=re.MULTILINE)

    # Replace tag IDs in the tags list
    def replace_tag(m):
        nonlocal changed
        val = m.group(1).strip().strip("'\"")
        if val in tag_map:
            changed = True
            return f"- {tag_map[val]}"
        return m.group(0)

    frontmatter = re.sub(r"^- (.+)$", replace_tag, frontmatter, flags=re.MULTILINE)

    if changed:
        write_file(path, f"---{frontmatter}---{body}")

    return changed


def migrate_config(config_path, epic_map, tag_map):
    content = read_file(config_path)
    lines = content.splitlines(keepends=True)
    result = []
    in_epics = False
    in_tags = False

    for line in lines:
        if re.match(r"^epics:", line):
            in_epics = True
            in_tags = False
        elif re.match(r"^tags:", line):
            in_tags = True
            in_epics = False
        elif line and not line.startswith(" ") and not line.startswith("-"):
            in_epics = False
            in_tags = False

        # Drop id: lines inside epics or tags sections
        # "- id: ..." becomes "-" (keep list marker), "  id: ..." is dropped entirely
        if (in_epics or in_tags) and re.match(r"^-\s+id:\s*", line):
            result.append("-\n")
            continue
        if (in_epics or in_tags) and re.match(r"^\s+id:\s*", line):
            continue

        result.append(line)

    write_file(config_path, "".join(result))


def main():
    config = read_file(CONFIG_PATH)

    epic_map = parse_id_name_map(config, "epics")
    tag_map = parse_id_name_map(config, "tags")

    print(f"Epics: {epic_map}")
    print(f"Tags:  {tag_map}")

    if not epic_map and not tag_map:
        print("Nothing to migrate — no ID-based epics or tags found.")
        return

    migrated = 0
    for fname in os.listdir(CARDS_DIR):
        if fname.endswith(".md"):
            path = os.path.join(CARDS_DIR, fname)
            if migrate_card(path, epic_map, tag_map):
                print(f"  migrated {fname}")
                migrated += 1

    print(f"{migrated} cards migrated.")

    migrate_config(CONFIG_PATH, epic_map, tag_map)
    print("Config rewritten without IDs.")


if __name__ == "__main__":
    main()
