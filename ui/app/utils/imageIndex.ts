export interface ImageIndex {
  startYear: number;
  startMonth: number;
  endYear: number;
  endMonth: number;
  folders: number[];
}

const pad2 = (n: number) => String(n).padStart(2, "0");

export function pickRandomImage(index: ImageIndex) {
  let total = 0;
  for (const c of index.folders) total += c;

  if (total === 0) {
    throw new Error("ImageIndex contains no images");
  }

  let r = Math.floor(Math.random() * total);

  let monthOffset = 0;
  for (let i = 0; i < index.folders.length; i++) {
    const c = index.folders[i];
    if (r < c) {
      monthOffset = i;
      break;
    }
    r -= c;
  }

  const startAbs = index.startYear * 12 + (index.startMonth - 1);
  const abs = startAbs + monthOffset;

  return `/b/${Math.floor(abs / 12)}-${pad2((abs % 12) + 1)}/${pad2(r + 1)}.webp`;
}

const images = import.meta.glob("../../public/b/*/*.webp", { eager: true });

export const imageIndex: ImageIndex = (() => {
  const counts = new Map<number, number>(); // key = year * 12 + (month - 1)

  for (const path in images) {
    const m = /\/b\/(\d{4})-(\d{2})\//.exec(path);
    if (!m) continue;

    const year = Number(m[1]);
    const month = Number(m[2]);
    const key = year * 12 + (month - 1);

    counts.set(key, (counts.get(key) ?? 0) + 1);
  }

  if (counts.size === 0) {
    throw new Error("No images found under /public/b");
  }

  const keys = [...counts.keys()].sort((a, b) => a - b);

  const startKey = keys[0];
  const endKey = keys[keys.length - 1];

  const startYear = Math.floor(startKey / 12);
  const startMonth = (startKey % 12) + 1;
  const endYear = Math.floor(endKey / 12);
  const endMonth = (endKey % 12) + 1;

  const totalMonths = endKey - startKey + 1;
  const folders = new Array<number>(totalMonths).fill(0);

  for (const [key, count] of counts) {
    folders[key - startKey] = count;
  }

  return {
    startYear,
    startMonth,
    endYear,
    endMonth,
    folders,
  };
})();
