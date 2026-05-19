import struct
from pathlib import Path

import ijson


def build_item_index(archive_files, index_path):
    file_map = [Path(p).name for p in archive_files]
    file_to_id = {name: i for i, name in enumerate(file_map)}

    with open(index_path, "wb") as out:
        out.write(struct.pack("<I", len(file_map)))

        for fname in file_map:
            encoded = fname.encode("utf-8")
            out.write(struct.pack("<H", len(encoded)))
            out.write(encoded)

        for archive_path in map(Path, archive_files):
            file_id = file_to_id[archive_path.name]
            print(f"Indexing {archive_path}...")

            with open(archive_path, "rb") as f:
                for obj in ijson.items(f, "item"):
                    gid = obj.get("gid")
                    if isinstance(gid, int):
                        out.write(struct.pack("<QHQq", gid, file_id, 0, 0))
                    else:
                        print("SKIPPING", obj)


if __name__ == "__main__":
    import sys

    archive_dir = Path("archive")
    index_path = Path("item_index.bin")

    if not archive_dir.exists() or not archive_dir.is_dir():
        print("archive/ directory not found")
        sys.exit(1)

    archive_files = sorted(archive_dir.glob("*.json"))

    if not archive_files:
        print("No JSON files found in archive/")
        sys.exit(1)

    print(f"Indexing {len(archive_files)} archive files...")
    build_item_index(archive_files, index_path)
    print(f"Index written to {index_path}")
