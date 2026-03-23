#!/usr/bin/env python3
# migrate_status_ids.py
#
# Rewrites card files to replace status IDs with human-readable names.
# Run from the project root (the folder containing rojekti/).
# Review the output, then check `git diff` before committing.

import os
import re

MAPPING = {
    "backlog":               "BACKLOG",
    "todo":                  "TO DO",
    "in-progress":           "DOING",
    "done":                  "DONE",
    "status-1774045884765":  "ARCHIVE",
}

cards_dir = os.path.join("rojekti", "cards")

if not os.path.isdir(cards_dir):
    print(f"ERROR: Could not find {cards_dir} — run this script from the project root.")
    exit(1)

updated = 0
skipped = 0
unknown = []

for filename in sorted(os.listdir(cards_dir)):
    if not filename.endswith(".md"):
        continue

    path = os.path.join(cards_dir, filename)
    with open(path, "r", encoding="utf-8") as f:
        content = f.read()

    # Split on --- to isolate frontmatter. A valid card file is:
    # "" | "---\n" | <yaml> | "---\n" | <body>
    # splitn(3) gives ["", yaml, body]
    parts = content.split("---", 2)
    if len(parts) < 3:
        print(f"  SKIP (invalid frontmatter): {filename}")
        skipped += 1
        continue

    frontmatter = parts[1]

    match = re.search(r"^status: (.+)$", frontmatter, flags=re.MULTILINE)
    if not match:
        print(f"  SKIP (no status field): {filename}")
        skipped += 1
        continue

    current = match.group(1).strip()

    if current not in MAPPING:
        unknown.append((filename, current))
        continue

    new_name = MAPPING[current]

    if current == new_name:
        skipped += 1
        continue

    new_frontmatter = re.sub(
        r"^status: .+$",
        f"status: {new_name}",
        frontmatter,
        flags=re.MULTILINE
    )

    new_content = "---".join([parts[0], new_frontmatter, parts[2]])

    with open(path, "w", encoding="utf-8") as f:
        f.write(new_content)

    print(f"  {filename}: {current!r} → {new_name!r}")
    updated += 1

print(f"\nDone. {updated} card(s) updated, {skipped} unchanged.")

if unknown:
    print(f"\nWARNING: {len(unknown)} card(s) had an unrecognised status — not touched:")
    for filename, status in unknown:
        print(f"  {filename}: {status!r}")
    print("Add these to MAPPING in the script if they need migrating.")
