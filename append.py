import json
import os
import re
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Iterator, Union

import ijson

MAX_ITEMS = 40_000
ARCHIVE_RE = re.compile(r"archive_(\d+)\.json")


def count_items(path):
    with open(path, "r", encoding="utf-8") as f:
        return sum(1 for _ in ijson.items(f, "item"))


def find_last_archive():
    files = []
    for name in os.listdir("./archive"):
        m = ARCHIVE_RE.fullmatch(name)
        if m:
            files.append((int(m.group(1)), name))

    if not files:
        return None, 0

    files.sort()
    return (Path("./archive") / files[-1][1]), files[-1][0]


def _open_text_auto(path: Union[str, Path]):
    p = Path(path)
    return open(p, "r", encoding="utf-8")


@dataclass(frozen=True)
class GiantJsonArrayLoader:
    prefix: str = "item"

    def __call__(self, source: Union[str, Path]) -> Iterator[Any]:
        with _open_text_auto(source) as f:
            yield from ijson.items(f, self.prefix)


_NUM_JSON_RE = re.compile(r"^(\d+)\.json$")


@dataclass(frozen=True)
class NumberedJsonFilesLoader:
    def __call__(self, source: Union[str, Path]) -> Iterator[Any]:
        src = Path(source)
        directory = src if src.is_dir() else src.parent

        numbered: list[tuple[int, Path]] = []
        for p in directory.iterdir():
            if not p.is_file():
                continue
            m = _NUM_JSON_RE.match(p.name)
            if m:
                numbered.append((int(m.group(1)), p))

        numbered.sort(key=lambda t: t[0])

        prev_path: Path | None = None

        try:
            for _, path in numbered:
                if prev_path is not None:
                    try:
                        prev_path.unlink()
                    except FileNotFoundError:
                        pass

                with path.open("r", encoding="utf-8") as f:
                    item = json.load(f)

                prev_path = path
                yield item

        finally:
            if prev_path is not None:
                try:
                    prev_path.unlink()
                except FileNotFoundError:
                    pass


def append_items(loader, input):
    last_file, last_index = find_last_archive()

    if last_file is None:
        raise ValueError("No existing archive found")
    else:
        current_index = last_index
        current_file = last_file
        current_count = count_items(current_file)

    items = loader(input)

    for obj in items:
        if current_count >= MAX_ITEMS:
            current_index += 1
            current_file = f"archive/archive_{current_index}.json"
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

    append_items(NumberedJsonFilesLoader(), sys.argv[1])
    print("Append completed.")
