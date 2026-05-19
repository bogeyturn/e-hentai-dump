import json
import math
import os
import re
import time
from concurrent.futures import ThreadPoolExecutor, as_completed
from dataclasses import dataclass
from io import BytesIO
from pathlib import Path
from typing import Any, Callable, Literal, Optional

import numpy as np
import requests
from PIL import Image


@dataclass
class GGParsed:
    b: Optional[str]
    default_o: int
    o_value: Optional[int]
    cases: list[int]

    def apply_m(self, g: int) -> int:
        if g in self.cases:
            return self.o_value
        return self.default_o


_B_RE = re.compile(r"""\bb\s*:\s*(['"])(?P<b>.*?)(?<!\\)\1\s*[,}]""", re.DOTALL)
_M_BODY_RE = re.compile(
    r"""\bm\s*:\s*function\s*\(\s*g\s*\)\s*\{\s*(?P<body>.*?)\s*\}\s*,\s*""",
    re.DOTALL,
)
_SWITCH_BODY_RE = re.compile(
    r"""\bswitch\s*\(\s*g\s*\)\s*\{\s*(?P<sw>.*?)\s*\}""", re.DOTALL
)
_DEFAULT_O_RE = re.compile(r"""\bvar\s+o\s*=\s*(?P<o>-?\d+)\s*;""")
_CASE_RE = re.compile(r"""\bcase\s+(?P<n>\d+)\s*:""")
_ASSIGN_BREAK_RE = re.compile(r"""\bo\s*=\s*(?P<o>-?\d+)\s*;\s*break\s*;""")


def parse_gg() -> GGParsed:
    ts_ms = int(time.time() * 1000)
    url = f"https://ltn.gold-usergeneratedcontent.net/gg.js?_={ts_ms}"
    headers = {
        "User-Agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) "
        "AppleWebKit/537.36 (KHTML, like Gecko) "
        "Chrome/122.0.0.0 Safari/537.36",
        "Accept": "*/*",
    }

    r = requests.get(url, headers=headers, timeout=30)
    r.raise_for_status()
    js_text = r.text
    b_match = _B_RE.search(js_text)
    b_val = b_match.group("b") if b_match else None

    m_match = _M_BODY_RE.search(js_text)
    if not m_match:
        raise ValueError("Could not find m: function(g) { ... } block.")
    m_body = m_match.group("body")

    default_o_match = _DEFAULT_O_RE.search(m_body)
    default_o = int(default_o_match.group("o")) if default_o_match else 0

    sw_match = _SWITCH_BODY_RE.search(m_body)
    if not sw_match:
        return GGParsed(b=b_val, default_o=default_o, o_value=None, cases=[])

    sw = sw_match.group("sw")

    cases = [int(m.group("n")) for m in _CASE_RE.finditer(sw)]

    assigned = [int(m.group("o")) for m in _ASSIGN_BREAK_RE.finditer(sw)]
    unique = sorted(set(assigned))

    if not unique:
        o_value = None
    elif len(unique) == 1:
        o_value = unique[0]
    else:
        raise ValueError(f"Found multiple different o assignments in switch: {unique}")

    return GGParsed(b=b_val, default_o=default_o, o_value=o_value, cases=cases)


def get_url(
    gg: GGParsed, hash: str, base: str | None, dir: Literal["webp", "avif"], big: bool
):
    g = int(hash[-1] + hash[-3:-1], 16)
    m = gg.apply_m(g)
    if base is None:
        retval = "w" if dir == "webp" else "a"
        retval = retval + str(1 + m)
    else:
        retval = chr(ord("a") + m) + base

    if base:
        url = f"https://{retval}.gold-usergeneratedcontent.net/{dir}{'big' if big else 'small'}tn/{hash[-1]}/{hash[-3:-1]}/{hash}.{dir}"
    else:
        url = f"https://{retval}.gold-usergeneratedcontent.net/{gg.b}{g}/{hash}.{dir}"
    return url


def load_img(gg, hash: str, thumb: bool):
    url = get_url(gg, hash, "tn" if thumb else None, "avif", False)
    headers = {
        "User-Agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36",
        "Referer": "https://hitomi.la/reader/3766408.html/",
        "Accept": "*/*",
    }

    r = requests.get(url, headers=headers, timeout=30)
    r.raise_for_status()
    with Image.open(BytesIO(r.content)) as im:
        im = im.convert("RGBA")
        w, h = im.size
        if w > 2000:
            im = im.resize((2000, int(h * 2000 / w)), Image.LANCZOS)
        arr = np.array(im, dtype=np.uint8, copy=False)
        if not arr.flags["C_CONTIGUOUS"]:
            arr = np.ascontiguousarray(arr)
        return arr


def choose_target_width(
    items: list[dict], clamp_min: int = 2048, clamp_max: int = 16384
) -> int:
    """
    Heuristic: make atlas width ~ sqrt(total_area), clamped.
    This gives a roughly square atlas without you having to pick a width.
    """
    total_area = 0
    max_w = 0
    for it in items:
        w = int(it["width"])
        h = int(it["height"])
        total_area += w * h
        max_w = max(max_w, w)

    if total_area <= 0:
        return clamp_min

    target = int(math.sqrt(total_area))
    target = max(target, max_w)
    target = max(target, clamp_min)
    target = min(target, clamp_max)
    return target


def pack_shelf(
    items: list[dict],
    atlas_width: int,
    padding: int = 2,
    sort_key: str = "height",
) -> tuple[int, int, dict[str, dict]]:
    items_sorted = sorted(items, key=lambda d: int(d[sort_key]), reverse=True)

    x = padding
    y = padding
    row_h = 0

    placements: dict[str, dict] = {}

    used_w = 0
    used_h = 0

    for it in items_sorted:
        w = int(it["width"])
        h = int(it["height"])
        key = it["hash"]

        if x + w + padding > atlas_width and x > padding:
            used_w = max(used_w, x)
            y += row_h + padding
            x = padding
            row_h = 0

        placements[key] = {
            "x": x,
            "y": y,
            "w": w,
            "h": h,
            "name": it.get("name"),
        }

        x += w + padding
        row_h = max(row_h, h)

        used_w = max(used_w, x)
        used_h = max(used_h, y + row_h + padding)

    atlas_h = used_h
    atlas_w = max(atlas_width, used_w)

    return atlas_w, atlas_h, placements


from dataclasses import dataclass


@dataclass
class AtlasLayout:
    index: int
    width: int
    height: int
    placements: dict[str, dict]
    items: list[dict]


def pack_row_major_multi(
    items: list[dict],
    max_w: int = 16384,
    max_h: int = 16384,
    padding: int = 2,
) -> list[AtlasLayout]:
    """
    Row-major packing in *input order*:
    - place at (0,0)
    - advance x by w + padding
    - if next item doesn't fit in width -> new row
    - if new row would exceed max_h -> new atlas
    """
    layouts: list[AtlasLayout] = []

    x = 0
    y = 0
    row_h = 0

    cur_items: list[dict] = []
    cur_places: dict[str, dict] = {}
    used_h = 0

    def finalize_current(atlas_index: int):
        nonlocal layouts, cur_items, cur_places, x, y, row_h, used_h
        if not cur_items:
            return
        height = max(1, used_h)
        layouts.append(
            AtlasLayout(
                index=atlas_index,
                width=max_w,
                height=height,
                placements=cur_places,
                items=cur_items,
            )
        )
        x = 0
        y = 0
        row_h = 0
        used_h = 0
        cur_items = []
        cur_places = {}

    atlas_index = 0

    for it in items:
        w = int(it["width"])
        h = int(it["height"])
        key = it["hash"]

        if w <= 0 or h <= 0:
            raise ValueError(f"Invalid dimensions for {key}: {w}x{h}")

        if w > max_w or h > max_h:
            raise ValueError(
                f"Item {key} is too large for atlas limit {max_w}x{max_h}: {w}x{h}"
            )

        if x > 0 and (x + w) > max_w:
            x = 0
            y = y + row_h + padding
            row_h = 0

        if y > 0 and (y + h) > max_h:
            finalize_current(atlas_index)
            atlas_index += 1

        cur_places[key] = {
            "x": x,
            "y": y,
            "w": w,
            "h": h,
            "name": it.get("name"),
        }
        cur_items.append(it)

        x = x + w + padding
        row_h = max(row_h, h)
        used_h = max(used_h, y + row_h)

    finalize_current(atlas_index)
    return layouts


def compose_atlas_numpy(
    items: list[dict],
    placements: dict[str, dict],
    images: dict[str, np.ndarray],
    atlas_w: int,
    atlas_h: int,
    background=(0, 0, 0, 0),
) -> Image.Image:
    atlas_arr = np.zeros((atlas_h, atlas_w, 4), dtype=np.uint8)
    if background != (0, 0, 0, 0):
        atlas_arr[:] = tuple(int(x) for x in background)

    for it in items:
        hsh = it["hash"]
        pl = placements[hsh]
        x, y = pl["x"], pl["y"]

        arr = images[hsh]
        hh, ww = arr.shape[:2]

        w_exp, h_exp = int(it["width"]), int(it["height"])
        w = min(w_exp, ww, atlas_w - x)
        h = min(h_exp, hh, atlas_h - y)

        if w > 0 and h > 0:
            atlas_arr[y : y + h, x : x + w, :] = arr[:h, :w, :]

    return Image.fromarray(atlas_arr, "RGBA")


def _save_image(img: Image.Image, path: Path):
    """
    Save based on suffix. Keep it simple & predictable.
    """
    suf = path.suffix.lower()
    path.parent.mkdir(parents=True, exist_ok=True)
    if suf == ".webp":
        img.save(path, format="WEBP", quality=80, method=3)
    elif suf == ".png":
        img.save(path, format="PNG")
    else:
        img.save(path)


def build_atlases_parallel(
    items: list[dict],
    out_image: Path,
    out_json: Path,
    load_img: Callable[[str], np.ndarray],
    padding: int = 2,
    background=(0, 0, 0, 0),
    max_workers: Optional[int] = None,
    max_w: int = 16384,
    max_h: int = 16384,
    measure_at_runtime: bool = True,
) -> dict[str, Any]:
    if not items:
        raise ValueError("items is empty")

    if max_workers is None:
        max_workers = min(32, (os.cpu_count() or 4) * 5)

    image_cache: dict[str, np.ndarray] | None = None

    if measure_at_runtime:
        image_cache = {}
        errors: dict[str, str] = {}

        with ThreadPoolExecutor(max_workers=max_workers) as ex:
            futures = {ex.submit(load_img, it["hash"]): it["hash"] for it in items}
            for fut in as_completed(futures):
                h = futures[fut]
                try:
                    arr = fut.result()
                    if arr.ndim < 2:
                        raise ValueError(f"Unexpected array shape: {arr.shape}")
                    hh, ww = arr.shape[:2]
                    if ww <= 0 or hh <= 0:
                        raise ValueError(f"Invalid runtime dimensions: {ww}x{hh}")

                    image_cache[h] = arr
                except Exception as e:
                    errors[h] = repr(e)

        if errors:
            msg = "Some images failed to download/measure:\n" + "\n".join(
                f"{h}: {err}" for h, err in list(errors.items())[:20]
            )
            raise RuntimeError(msg)

        for it in items:
            arr = image_cache[it["hash"]]
            hh, ww = arr.shape[:2]
            it["width"] = int(ww)
            it["height"] = int(hh)

    layouts = pack_row_major_multi(items, max_w=max_w, max_h=max_h, padding=padding)

    out_dir = out_image.parent
    stem = out_image.stem
    suf = out_image.suffix

    if len(layouts) == 1:
        atlas_files = [out_dir / (stem + suf)]
    else:
        atlas_files = [out_dir / f"{stem}_{i}{suf}" for i in range(len(layouts))]

    by_hash: dict[str, dict] = {}
    atlases_meta: list[dict[str, Any]] = []

    for layout in layouts:
        idx = layout.index
        atlas_path = atlas_files[idx]

        if image_cache is not None:
            images = {it["hash"]: image_cache[it["hash"]] for it in layout.items}
        else:
            images: dict[str, np.ndarray] = {}
            errors: dict[str, str] = {}
            with ThreadPoolExecutor(max_workers=max_workers) as ex:
                futures = {
                    ex.submit(load_img, it["hash"]): it["hash"] for it in layout.items
                }
                for fut in as_completed(futures):
                    h = futures[fut]
                    try:
                        images[h] = fut.result()
                    except Exception as e:
                        errors[h] = repr(e)

            if errors:
                msg = "Some images failed to download:\n" + "\n".join(
                    f"{h}: {err}" for h, err in list(errors.items())[:20]
                )
                raise RuntimeError(msg)

        atlas_img = compose_atlas_numpy(
            layout.items,
            layout.placements,
            images,
            layout.width,
            layout.height,
            background,
        )
        _save_image(atlas_img, atlas_path)

        atlases_meta.append(
            {
                "file": str(atlas_path.name),
                "width": layout.width,
                "height": layout.height,
                "padding": padding,
                "max_workers": max_workers,
                "index": idx,
                "measure_at_runtime": measure_at_runtime,
            }
        )

        for it in layout.items:
            hsh = it["hash"]
            pl = dict(layout.placements[hsh])
            pl["atlas"] = idx
            pl["file"] = str(atlas_path.name)
            by_hash[hsh] = pl

    manifest = {
        "atlases": atlases_meta,
        "by_hash": by_hash,
        "items": [
            {
                "hash": it["hash"],
                "name": it.get("name"),
                "width": int(it["width"]),
                "height": int(it["height"]),
                **by_hash[it["hash"]],
            }
            for it in items
        ],
    }

    out_json.parent.mkdir(parents=True, exist_ok=True)
    out_json.write_text(json.dumps(manifest, indent=2), encoding="utf-8")
    return manifest


def get_imgs(id: int):
    url = f"https://ltn.gold-usergeneratedcontent.net/galleries/{id}.js"
    headers = {
        "User-Agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) "
        "AppleWebKit/537.36 (KHTML, like Gecko) "
        "Chrome/122.0.0.0 Safari/537.36",
        "Accept": "*/*",
    }

    r = requests.get(url, headers=headers, timeout=30)
    r.raise_for_status()
    js_text = r.text
    text = js_text.replace("var galleryinfo = ", "")
    return json.loads(text)["files"]


if __name__ == "__main__":
    import argparse

    def build_parser():
        p = argparse.ArgumentParser(
            prog="img-atlas",
            description="Build a single atlas image + JSON offsets from a list of images.",
        )

        p.add_argument(
            "--gid",
            type=int,
            required=True,
            help="Start offset into the image list (default: 0).",
        )

        p.add_argument(
            "--offset",
            type=int,
            default=0,
            help="Start offset into the image list (default: 0).",
        )
        p.add_argument(
            "--size",
            type=int,
            default=50,
            help="How many images to include (default: 50).",
        )
        p.add_argument(
            "--out",
            type=Path,
            required=True,
            help="Output atlas image path (default: ./out/atlas.webp).",
        )
        p.add_argument(
            "--json-path",
            type=Path,
            required=True,
            help="Output JSON metadata path (default: ./out/atlas.json).",
        )
        p.add_argument(
            "--workers",
            type=int,
            default=20,
            help="Parallel download workers (default: 20).",
        )
        return p.parse_args()

    args = build_parser()

    t0 = time.perf_counter()

    with ThreadPoolExecutor(max_workers=2) as ex:
        fut1 = ex.submit(get_imgs, args.gid)
        fut2 = ex.submit(parse_gg)

        imgs = fut1.result()
        gg = fut2.result()

    imgs = [x for x in imgs[args.offset : args.offset + args.size]]

    def loader(it: str):
        pil_img = load_img(gg, it, True)

        return pil_img

    manifest = build_atlases_parallel(
        items=imgs,
        out_image=args.out,
        out_json=args.json_path,
        load_img=loader,
        padding=4,
        max_workers=args.workers,
        max_w=16383,
        max_h=16383,
    )
    t2 = time.perf_counter()

    print(f"{t2 - t0:.6f} seconds")
