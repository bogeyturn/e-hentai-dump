import json
import sys
import unittest
from pathlib import Path
from tempfile import TemporaryDirectory

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

from merge_detail_archive import merge_detail_into_archive


class MergeDetailArchiveTests(unittest.TestCase):
    def test_replaces_existing_appends_new_and_removes_detail_files(self):
        with TemporaryDirectory() as tmp:
            root = Path(tmp)
            archive = root / "archive"
            detail = root / "detail"
            archive.mkdir()
            detail.mkdir()
            (archive / "archive_1.json").write_text(
                json.dumps([{"gid": 1, "title": "old"}]), encoding="utf-8"
            )
            (detail / "1.json").write_text(
                json.dumps({"gid": 1, "title": "new"}), encoding="utf-8"
            )
            (detail / "2.json").write_text(
                json.dumps({"gid": 2, "title": "added"}), encoding="utf-8"
            )

            result = merge_detail_into_archive(root)

            items = json.loads((archive / "archive_1.json").read_text(encoding="utf-8"))
            self.assertEqual(items, [{"gid": 1, "title": "new"}, {"gid": 2, "title": "added"}])
            self.assertEqual(result.replaced, 1)
            self.assertEqual(result.appended, 1)
            self.assertEqual(result.deleted_detail_files, 2)
            self.assertFalse((detail / "1.json").exists())
            self.assertFalse((detail / "2.json").exists())
            self.assertTrue((root / "item_index.bin").exists())

    def test_leaves_detail_files_when_archive_update_fails_validation(self):
        with TemporaryDirectory() as tmp:
            root = Path(tmp)
            archive = root / "archive"
            detail = root / "detail"
            archive.mkdir()
            detail.mkdir()
            (archive / "archive_1.json").write_text("[]", encoding="utf-8")
            (detail / "broken.json").write_text(
                json.dumps({"title": "missing gid"}), encoding="utf-8"
            )

            with self.assertRaises(ValueError):
                merge_detail_into_archive(root)

            self.assertTrue((detail / "broken.json").exists())


if __name__ == "__main__":
    unittest.main()
