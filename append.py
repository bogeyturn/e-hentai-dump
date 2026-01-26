import json
import os
import re
import sys

import ijson

MAX_ITEMS = 40_000
ARCHIVE_RE = re.compile(r"archive/archive_(\d+)\.json")


def count_items(path):
    with open(path, "r", encoding="utf-8") as f:
        return sum(1 for _ in ijson.items(f, "item"))


def find_last_archive():
    files = []
    for name in os.listdir("."):
        m = ARCHIVE_RE.fullmatch(name)
        if m:
            files.append((int(m.group(1)), name))

    if not files:
        return None, 0

    files.sort()
    return files[-1][1], files[-1][0]


def append_items(input_json):
    last_file, last_index = find_last_archive()

    if last_file is None:
        raise ValueError("No existing archive found")
    else:
        current_index = last_index
        current_file = last_file
        current_count = count_items(current_file)

    with open(input_json, "r", encoding="utf-8") as f_in:
        items = ijson.items(f_in, "item")

        for obj in items:
            if current_count >= MAX_ITEMS:
                current_index += 1
                current_file = f"archive_{current_index}.json"
                current_count = 0

                with open(current_file, "w", encoding="utf-8") as f:
                    f.write("[\n]\n")

            with open(current_file, "rb+") as f:
                f.seek(-2, os.SEEK_END)
                if current_count > 0:
                    f.write(b",\n")
                else:
                    f.write(b"\n")

                json_str = json.dumps(
                    obj,
                    separators=(",", ":"),
                    sort_keys=True,
                    ensure_ascii=False,
                )
                f.write(json_str.encode("utf-8"))
                f.write(b"\n]")

            current_count += 1


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: append_to_archive.py <input.json>")
        sys.exit(1)

    append_items(sys.argv[1])
    print("Append completed.")
