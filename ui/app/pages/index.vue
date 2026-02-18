<script setup lang="ts">
import NavBar from "~/components/NavBar.vue";
import SearchArea from "@/components/home/SearchArea.vue";
import FoundItems from "@/components/home/FoundItems.vue";
import RangeBar from "@/components/home/RangeBar.vue";
import CommonView from "~/components/views/CommonView.vue";

import { getSession } from "@/utils/session";
import {
  boolean as parseBoolean,
  number as parseNumber,
  optional as parseOptional,
  string as parseString,
  useTypedQuery,
  withDefaults as parseWithDefaults,
} from "~/composables/useTypedQuery";
import { computed, useHead, useRoute, useRouter } from "#imports";
import type { AdvancedConfig, Info, SearchInfo } from "exx";
import InfoComponent from "~/components/info/InfoComponent.vue";

const ses = getSession();
const route = useRoute();
const router = useRouter();

useHead({
  title: "Search - ExHentai",
});

const view = useCookie("search-view", {
  default: () => "t",
});

const showUser = ref(getCookie("includeGalleries") == "1");
const showFav = ref(getCookie("includeFavGalleries") == "1");

const viewPage = ref<number | null>(null);
const galleryData = ref<Info | { error: unknown } | null>(null);
const galleryPage = ref(1);
const loaded = ref("");

type SearchPage = {
  items: SearchInfo[];
  first: boolean;
  last: boolean;
  progress_min: number;
  progress_max: number;
  count: string;
};

type SearchWindowSlot = SearchPage | null;
type GalleryKey = `${number}:${number}`;
type GalleryValue = Info;
type FetchDirection = "prev" | "next";

const searchWindow = ref<{ prev: SearchWindowSlot; current: SearchWindowSlot; next: SearchWindowSlot }>({
  prev: null,
  current: null,
  next: null,
});

const cookie = useCookie("EX_COOKIE");

const query = useTypedQuery<{
  f_search: string;
  prev?: number;
  next?: number;
  jump?: string;
  seek?: string;
  range?: number;
  f_sh?: boolean;
  f_sto?: boolean;
  f_spf?: number;
  f_spt?: number;
  f_srdd?: number;
  f_sfl?: boolean;
  f_sfu?: boolean;
  f_sft?: boolean;
  f_cats?: number;
}>({
  prev: parseOptional(parseNumber()),
  next: parseOptional(parseNumber()),
  f_search: parseWithDefaults(parseString(), ""),
  jump: parseOptional(parseString()),
  seek: parseOptional(parseString()),
  range: parseOptional(parseNumber()),
  f_sh: parseOptional(parseBoolean()),
  f_sto: parseOptional(parseBoolean()),
  f_spf: parseOptional(parseNumber()),
  f_spt: parseOptional(parseNumber()),
  f_srdd: parseOptional(parseNumber()),
  f_sfl: parseOptional(parseBoolean()),
  f_sfu: parseOptional(parseBoolean()),
  f_sft: parseOptional(parseBoolean()),
  f_cats: parseOptional(parseNumber()),
});

function asFlag(v: boolean | undefined) {
  if (v === undefined) return null;
  return v;
}

const searchParams = computed(() => {
  if (!query.value.data) return null;
  const d = query.value.data;

  const advanced: AdvancedConfig = {
    explunged: asFlag(d.f_sh),
    require_torrent: asFlag(d.f_sto),
    min_pages: d.f_spf ?? null,
    max_pages: d.f_spt ?? null,
    min_rating: d.f_srdd && d.f_srdd > 0 ? d.f_srdd : null,
    disable_lang: asFlag(d.f_sfl),
    disable_uploader: asFlag(d.f_sfu),
    disable_tags: asFlag(d.f_sft),
  };

  return {
    query: d.f_search,
    prev: d.prev,
    next: d.next,
    jump: d.jump,
    seek: d.seek,
    range: d.range,
    categories: d.f_cats && d.f_cats > 0 ? d.f_cats : null,
    advanced,
  };
});

const itemsError = computed(() => {
  if (query.value.error) return query.value.error;
  if (items.value && "error" in items.value) {
    return String(items.value.error);
  }
  return null;
});

const { data: items } = await useAsyncData(
  () => {
    const p = searchParams.value;
    return [
      "search",
      p?.query ?? "",
      p?.prev ?? "",
      p?.next ?? "",
      p?.range ?? "",
      p?.seek ?? "",
      p?.jump ?? "",
      p?.categories ?? "",
      p?.advanced.explunged ?? "",
      p?.advanced.require_torrent ?? "",
      p?.advanced.min_pages ?? "",
      p?.advanced.max_pages ?? "",
      p?.advanced.min_rating ?? "",
      p?.advanced.disable_lang ?? "",
      p?.advanced.disable_uploader ?? "",
      p?.advanced.disable_tags ?? "",
    ].join("-");
  },
  async () => {
    const p = searchParams.value;
    if (!p) {
      return { error: query.value.error ?? "Invalid query parameters" };
    }

    const pid = p.prev ?? p.next;
    const forward = !p.prev;

    try {
      const s = await ses.search(
        p.query,
        pid !== undefined ? BigInt(pid) : null,
        p.range ?? null,
        p.seek ?? null,
        p.jump ?? null,
        forward,
        p.advanced,
        p.categories,
        true,
      );
      const cookie_val = await ses.cookie();
      if (cookie_val != cookie.value) cookie.value = cookie_val;
      return s;
    } catch (err) {
      return { error: err };
    }
  },
  { server: true, immediate: true },
);

const galleryCache = ref<Map<GalleryKey, GalleryValue>>(new Map());

function filterItems(list: SearchInfo[]) {
  let a = list;
  if (!showFav.value) {
    a = a.filter((item) => item.favorite == undefined && !item.voted);
  }
  if (!showUser.value) {
    a = a.filter((item) => item.publisher !== "username2983740");
  }
  return a;
}

function isSearchPage(v: unknown): v is SearchPage {
  return Boolean(v) && typeof v === "object" && "items" in (v as object) && Array.isArray((v as SearchPage).items);
}

async function fetchSearchPage(anchorId: number, direction: boolean): Promise<SearchPage | null> {
  const p = searchParams.value;
  if (!p) return null;

  try {
    const page = await ses.search(
      p.query,
      BigInt(anchorId),
      p.range ?? null,
      p.seek ?? null,
      p.jump ?? null,
      direction,
      p.advanced,
      p.categories,
      true,
    );
    const cookie_val = await ses.cookie();
    if (cookie_val != cookie.value) cookie.value = cookie_val;
    return page as SearchPage;
  } catch {
    return null;
  }
}

function clearSearchWindow() {
  searchWindow.value = { prev: null, current: null, next: null };
}

async function hydrateSearchWindow() {
  clearSearchWindow();
  if (!items.value || "error" in items.value || !isSearchPage(items.value)) {
    return;
  }

  searchWindow.value.current = items.value;

  const current = searchWindow.value.current;
  if (!current || current.items.length === 0) return;

  const firstId = current.items[0]?.id;
  const lastItemId = current.items[current.items.length - 1]?.id;

  const tasks: Promise<void>[] = [];

  if (!current.first && typeof firstId === "number") {
    tasks.push(
      fetchSearchPage(firstId, false).then((page) => {
        searchWindow.value.prev = page;
      }),
    );
  }

  if (!current.last && typeof lastItemId === "number") {
    tasks.push(
      fetchSearchPage(lastItemId, true).then((page) => {
        searchWindow.value.next = page;
      }),
    );
  }

  await Promise.all(tasks);
}

async function ensureWindowEdge(direction: FetchDirection) {
  const current = searchWindow.value.current;
  if (!current || current.items.length === 0) return;

  if (direction === "next") {
    if (searchWindow.value.next) return;
    if (current.last) return;
    const anchor = current.items[current.items.length - 1]?.id;
    if (typeof anchor !== "number") return;
    searchWindow.value.next = await fetchSearchPage(anchor, true);
    return;
  }

  if (searchWindow.value.prev) return;
  if (current.first) return;
  const anchor = current.items[0]?.id;
  if (typeof anchor !== "number") return;
  searchWindow.value.prev = await fetchSearchPage(anchor, false);
}

async function shiftWindow(direction: FetchDirection) {
  await ensureWindowEdge(direction);

  if (direction === "next") {
    const oldCurrent = searchWindow.value.current;
    const oldNext = searchWindow.value.next;
    if (!oldCurrent || !oldNext) return false;

    searchWindow.value.prev = oldCurrent;
    searchWindow.value.current = oldNext;
    searchWindow.value.next = null;

    await ensureWindowEdge("next");
    return true;
  }

  const oldCurrent = searchWindow.value.current;
  const oldPrev = searchWindow.value.prev;
  if (!oldCurrent || !oldPrev) return false;

  searchWindow.value.next = oldCurrent;
  searchWindow.value.current = oldPrev;
  searchWindow.value.prev = null;

  await ensureWindowEdge("prev");
  return true;
}

const resultsFiltered = computed(() => {
  if (!searchWindow.value.current) return [];
  return filterItems(searchWindow.value.current.items);
});

function getGalleryErrorText() {
  if (!galleryData.value || !("error" in galleryData.value)) return "";
  const err = galleryData.value.error;
  if (typeof err === "string") return err;
  if (err && typeof err === "object" && "message" in err) {
    const msg = (err as { message?: unknown }).message;
    if (typeof msg === "string") return msg;
  }
  return String(err ?? "");
}

const isCopyrightStriked = computed(() => {
  const text = getGalleryErrorText().toLowerCase();
  return text.includes("copyright");
});

function makeCacheKey(id: number, page: number): GalleryKey {
  return `${id}:${page}`;
}

function findItemById(id: number): SearchInfo | null {
  const slots: Array<SearchWindowSlot> = [
    searchWindow.value.prev,
    searchWindow.value.current,
    searchWindow.value.next,
  ];

  for (const slot of slots) {
    if (!slot) continue;
    const found = slot.items.find((v) => v.id == id);
    if (found) return found;
  }

  return null;
}

async function loadGallery(id: number, page: number, cache: boolean) {
  const key = makeCacheKey(id, page);
  if (galleryCache.value.has(key)) {
    if (!cache) {
      galleryData.value = galleryCache.value.get(key) ?? null;
      loaded.value = key;
    }
    return;
  }

  try {
    const item = findItemById(id);
    if (!item) return;
    if (!cache && loaded.value == key) {
      return;
    }

    const d = await ses.info(BigInt(id), item.token, page, true);

    const cookie_val = await ses.cookie();
    if (cookie_val != cookie.value) cookie.value = cookie_val;

    galleryCache.value.set(key, d);

    if (!cache) {
      loaded.value = key;
      galleryData.value = d;
    }

    if (galleryCache.value.size > 24) {
      const firstKey = galleryCache.value.keys().next().value;
      if (firstKey) galleryCache.value.delete(firstKey);
    }
  } catch (e) {
    if (!cache) {
      galleryData.value = { error: e };
    }
  }
}

async function openGallery(id: number, page = 1) {
  viewPage.value = id;
  galleryPage.value = page;
  window.scrollTo({ top: 0, behavior: "instant" });
  await loadGallery(id, page, false);
  await primeNeighborGalleries();
}

async function primeNeighborGalleries() {
  const currentId = viewPage.value;
  if (!currentId) return;
  const next = nextId();
  const prev = prevId();

  if (next) await loadGallery(next, 1, true);
  if (prev) await loadGallery(prev, 1, true);
}

watch(galleryPage, async (newValue) => {
  if (newValue !== null && viewPage.value) {
    await loadGallery(viewPage.value, newValue, false);
  }
});

watch(
  () => [items.value, showFav.value, showUser.value],
  async () => {
    await hydrateSearchWindow();

    if (!viewPage.value) return;
    if (findItemById(viewPage.value)) return;

    const fallback = resultsFiltered.value[0]?.id ?? null;
    if (fallback) {
      await openGallery(fallback, 1);
      return;
    }

    viewPage.value = null;
    galleryData.value = null;
  },
  { immediate: true },
);

function prevId() {
  const currentId = viewPage.value;
  if (currentId === null) return null;
  const visible = resultsFiltered.value;
  const index = visible.findIndex((v) => v.id == currentId);
  if (index <= 0) return null;
  return visible[index - 1]?.id ?? null;
}

function nextId() {
  const currentId = viewPage.value;
  if (currentId === null) return null;
  const visible = resultsFiltered.value;
  const index = visible.findIndex((v) => v.id == currentId);
  if (index == -1 || index >= visible.length - 1) {
    return null;
  }
  return visible[index + 1]?.id ?? null;
}

async function moveWithinInfo(direction: FetchDirection) {
  const id = direction === "next" ? nextId() : prevId();
  if (id) {
    await openGallery(id, 1);
    return;
  }

  const shifted = await shiftWindow(direction);
  if (!shifted) return;
  syncSearchUrlAfterShift(direction);

  const visible = resultsFiltered.value;
  if (!visible.length) return;

  const target =
    direction === "next" ? visible[0]?.id : visible[visible.length - 1]?.id;
  if (target) {
    await openGallery(target, 1);
  }
}

function syncSearchUrlAfterShift(direction: FetchDirection) {
  const current = searchWindow.value.current;
  if (!current) return;

  if (current.first) {
    void router.replace({
      path: route.path,
      query: {
        ...route.query,
        prev: undefined,
        next: undefined,
      },
    });
    return;
  }

  if (direction === "next") {
    const anchor = searchWindow.value.prev?.items[searchWindow.value.prev.items.length - 1]?.id;
    if (!anchor) return;
    void router.replace({
      path: route.path,
      query: {
        ...route.query,
        prev: undefined,
        next: anchor,
      },
    });
    return;
  }

  const anchor = searchWindow.value.next?.items[0]?.id;
  if (!anchor) return;
  void router.replace({
    path: route.path,
    query: {
      ...route.query,
      next: undefined,
      prev: anchor,
    },
  });
}

function setNextId() {
  void moveWithinInfo("next");
}

const isFullscreen = ref(false);

function updateFullscreenState() {
  if (typeof document === "undefined") return;
  isFullscreen.value = Boolean(document.fullscreenElement);
}

function handleInfoHotkeys(e: KeyboardEvent) {
  if (!viewPage.value) return;

  const target = e.target as HTMLElement | null;
  const tag = target?.tagName?.toLowerCase();
  const isTyping =
    tag === "input" || tag === "textarea" || target?.isContentEditable;

  if (isTyping) return;

  if (e.key === "ArrowRight") {
    e.preventDefault();
    void moveWithinInfo("next");
  }

  if (e.key === "ArrowLeft") {
    e.preventDefault();
    void moveWithinInfo("prev");
  }
}

onMounted(() => {
  updateFullscreenState();
  document.addEventListener("fullscreenchange", updateFullscreenState);
  window.addEventListener("keydown", handleInfoHotkeys);
});

onUnmounted(() => {
  document.removeEventListener("fullscreenchange", updateFullscreenState);
  window.removeEventListener("keydown", handleInfoHotkeys);
});

watch(
  [viewPage, isFullscreen],
  async ([id, fullscreen]) => {
    if (!id || !fullscreen) return;
    await ensureWindowEdge("next");
    await primeNeighborGalleries();
  },
  { immediate: true },
);

const displayItems = computed(() => searchWindow.value.current);
const paginationFirst = computed(() => displayItems.value?.first ?? true);
const paginationLast = computed(() => (displayItems.value?.last ? null : "1"));
const paginationPrev = computed(() => {
  if (
    !displayItems.value ||
    displayItems.value.first ||
    displayItems.value.items.length === 0
  ) {
    return null;
  }
  return displayItems.value.items[0]?.id ?? null;
});
const paginationNext = computed(() => {
  if (
    !displayItems.value ||
    displayItems.value.last ||
    displayItems.value.items.length === 0
  ) {
    return null;
  }
  return displayItems.value.items[displayItems.value.items.length - 1]?.id ?? null;
});
</script>

<template>
  <div>
    <NavBar />
    <div v-if="itemsError">{{ itemsError }}</div>
    <template v-if="viewPage">
      <InfoComponent
        v-if="galleryData && !('error' in galleryData)"
        v-model:page="galleryPage"
        :gallery-data="galleryData"
        @close="() => setNextId()"
      />
      <div v-else>
        <template v-if="isCopyrightStriked">
          <div>Copyright striked</div>
          <button type="button" @click="() => setNextId()">Next</button>
        </template>
        <template v-else>{{ galleryData?.error }} <button type="button" @click="() => setNextId()">Next</button></template>
      </div>
    </template>
    <template v-else>
      <div v-if="displayItems && displayItems.items.length === 0">No items found</div>
      <div
        v-if="displayItems && displayItems.items.length > 0 && !itemsError"
        class="ido"
        :style="view == 't' ? 'max-width: var(--container-width)' : ''"
      >
        <h1 class="ih">
          E-Hentai Galleries: The Free Hentai Doujinshi, Manga and Image Gallery
          System
        </h1>
        <SearchArea v-model:showuser="showUser" v-model:showfav="showFav" />
        <div v-if="displayItems && displayItems.items.length > 0" style="position: relative; z-index: 2">
          <RangeBar
            :progress_min="displayItems.progress_min"
            :progress_max="displayItems.progress_max"
          />
          <FoundItems :text="displayItems.count" />
          <CommonView
            :first="paginationFirst"
            :last="paginationLast"
            :query="searchParams?.query ?? ''"
            :prev="paginationPrev"
            :next="paginationNext"
            :items="resultsFiltered"
            @open-all="resultsFiltered[0] && openGallery(resultsFiltered[0].id, 1)"
          />
        </div>
      </div>
    </template>
  </div>
</template>

<style lang="css" src="~/styles/index.css" scoped></style>
