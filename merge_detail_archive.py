import argparse
import json
import os
from contextlib import contextmanager
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Iterator

from append import append_items
from build_item_index import build_item_index
from replace import load_index, replace_gids


@dataclass(frozen=True)
class MergeResult:
    replaced: int
    appended: int
    deleted_detail_files: int


@dataclass(frozen=True)
class InMemoryLoader:
    items: list[dict[str, Any]]

    def __call__(self, _source: object) -> Iterator[dict[str, Any]]:
        yield from self.items


def merge_detail_into_archive(root: Path | str = Path(".")) -> MergeResult:
    root = Path(root)
    archive_dir = root / "archive"
    detail_dir = root / "detail"
    index_path = root / "item_index.bin"

    if not archive_dir.is_dir():
        raise FileNotFoundError(f"{archive_dir} does not exist")
    if not detail_dir.is_dir():
        raise FileNotFoundError(f"{detail_dir} does not exist")

    detail_items = load_detail_items(detail_dir)
    if not detail_items:
        build_index(archive_dir, index_path)
        return MergeResult(replaced=0, appended=0, deleted_detail_files=0)

    build_index(archive_dir, index_path)
    gid_to_file = load_index(index_path)

    replacements: dict[Path, dict[int, dict[str, Any]]] = {}
    appendable: list[dict[str, Any]] = []

    for detail_path, item in detail_items:
        gid = item["gid"]
        archive_file = gid_to_file.get(gid)
        if archive_file is None:
            appendable.append(item)
        else:
            replacements.setdefault(archive_dir / archive_file, {})[gid] = item

    for archive_path, file_replacements in replacements.items():
        print(f"Replacing {len(file_replacements)} item(s) in {archive_path}")
        replace_gids(archive_path, file_replacements)

    if appendable:
        print(f"Appending {len(appendable)} new item(s)")
        with working_directory(root):
            append_items(InMemoryLoader(appendable), detail_dir)

    for detail_path, _ in detail_items:
        detail_path.unlink()

    build_index(archive_dir, index_path)

    return MergeResult(
        replaced=sum(len(items) for items in replacements.values()),
        appended=len(appendable),
        deleted_detail_files=len(detail_items),
    )


def build_index(archive_dir: Path, index_path: Path) -> None:
    archive_files = sorted(
        archive_dir.glob("archive_*.json"), key=lambda path: archive_number(path)
    )
    if not archive_files:
        raise ValueError(f"No archive JSON files found in {archive_dir}")
    build_item_index(archive_files, index_path)


def load_detail_items(detail_dir: Path) -> list[tuple[Path, dict[str, Any]]]:
    items = []
    for detail_path in sorted(detail_dir.glob("*.json"), key=lambda path: numeric_stem(path)):
        with detail_path.open("r", encoding="utf-8") as f:
            item = json.load(f)
        gid = item.get("gid")
        if not isinstance(gid, int):
            raise ValueError(f"{detail_path} has no integer gid")
        items.append((detail_path, item))
    return items


def archive_number(path: Path) -> int:
    stem = path.stem
    _, _, number = stem.partition("_")
    try:
        return int(number)
    except ValueError:
        return 2**31


def numeric_stem(path: Path) -> int:
    try:
        return int(path.stem)
    except ValueError:
        return 2**31


@contextmanager
def working_directory(path: Path):
    previous = Path.cwd()
    os.chdir(path)
    try:
        yield
    finally:
        os.chdir(previous)


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Merge detail JSON files into archive shards."
    )
    parser.add_argument("--root", type=Path, default=Path("."))
    args = parser.parse_args()

    result = merge_detail_into_archive(args.root)
    print(
        "Merged detail into archive: "
        f"{result.replaced} replaced, "
        f"{result.appended} appended, "
        f"{result.deleted_detail_files} detail file(s) removed."
    )


if __name__ == "__main__":
    main()
