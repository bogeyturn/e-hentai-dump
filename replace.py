import json
import os
import struct
import tempfile
from pathlib import Path

import ijson


def replace_gids(input_path: Path, replacements: dict, output_path=None):
    if not replacements:
        return

    if output_path is None:
        fd, temp_path = tempfile.mkstemp(prefix="json_replace_", suffix=".json")
        os.close(fd)
        output_path = temp_path

    found_gids = set()

    with (
        open(input_path, "r", encoding="utf-8") as f_in,
        open(output_path, "w", encoding="utf-8") as f_out,
    ):
        items = ijson.items(f_in, "item")
        f_out.write("[\n")
        first = True

        for obj in items:
            gid = obj.get("gid")
            if gid in replacements:
                obj = replacements[gid]
                found_gids.add(gid)

            json_str = json.dumps(
                obj, separators=(",", ":"), sort_keys=True, ensure_ascii=False
            )

            if not first:
                f_out.write(",\n")
            first = False
            f_out.write(json_str)

        f_out.write("\n]\n")

    missing_gids = set(replacements.keys()) - found_gids
    if missing_gids:
        print(f"Warning: gid(s) not found in {input_path}: {missing_gids}")

    if output_path != input_path:
        os.replace(output_path, input_path)


def load_index(index_path: Path):
    gid_to_file = {}
    with open(index_path, "rb") as f:
        file_count = struct.unpack("<I", f.read(4))[0]

        file_names = []
        for _ in range(file_count):
            name_len = struct.unpack("<H", f.read(2))[0]
            name = f.read(name_len).decode("utf-8")
            file_names.append(name)

        while True:
            entry = f.read(26)
            if not entry:
                break
            if len(entry) < 26:
                break

            gid, file_id, obj_start, size = struct.unpack("<QHQq", entry)
            file_name = file_names[file_id]
            gid_to_file[gid] = file_name
    return gid_to_file


if __name__ == "__main__":
    import sys

    if len(sys.argv) != 4:
        print(
            "Usage: replace_gids_indexed.py <archive_dir> <index_path> <replacements.json>"
        )
        sys.exit(1)

    archive_dir = Path(sys.argv[1])
    index_path = Path(sys.argv[2])
    replacements_file = Path(sys.argv[3])

    if not archive_dir.exists() or not archive_dir.is_dir():
        print(f"{archive_dir}/ directory not found")
        sys.exit(1)

    if not index_path.exists():
        print(f"{index_path} not found")
        sys.exit(1)

    if not replacements_file.exists():
        print(f"{replacements_file} not found")
        sys.exit(1)

    with open(replacements_file, "r", encoding="utf-8") as f:
        replacements_list = json.load(f)
    replacements_dict = {obj["gid"]: obj for obj in replacements_list}

    gid_to_file = load_index(index_path)

    file_replacements = {}
    for gid, obj in replacements_dict.items():
        file_name = gid_to_file.get(gid)
        if not file_name:
            print(f"Warning: gid {gid} not found in index, skipping")
            continue
        file_path = archive_dir / file_name
        file_replacements.setdefault(file_path, {})[gid] = obj

    for file_path, repls in file_replacements.items():
        print(f"Updating {file_path} ({len(repls)} items)...")
        replace_gids(file_path, repls)

    print("All replacements complete.")
