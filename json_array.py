import json

try:
    import ijson as _ijson
except ModuleNotFoundError:
    _ijson = None


def iter_items(file_obj, prefix="item"):
    if _ijson is not None:
        yield from _ijson.items(file_obj, prefix)
        return

    data = json.load(file_obj)
    if not isinstance(data, list):
        raise ValueError("Expected a top-level JSON array")
    yield from data
