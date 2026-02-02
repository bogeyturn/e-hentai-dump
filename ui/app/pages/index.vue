<script setup lang="ts">
import NavBar from "~/components/NavBar.vue";
import SearchArea from "@/components/home/SearchArea.vue";
import FoundItems from "@/components/home/FoundItems.vue";
import RangeBar from "@/components/home/RangeBar.vue";
import CommonView from "~/components/views/CommonView.vue";

import { getSession } from "@/utils/session";
import { computed, useHead, useRoute } from "#imports";
import type { SearchInfo } from "exx";
import InfoComponent from "~/components/info/InfoComponent.vue";

const route = useRoute();
const ses = getSession();

useHead({
  title: "Search - ExHentai",
});

const view = useCookie("search-view", {
  default: () => "t",
});

const showUser = ref(getCookie("includeGalleries") == "1");
const showFav = ref(getCookie("includeFavGalleries") == "1");

const viewPage = ref<number | null>(null);

const galleryData = ref(null);
const galleryPage = ref(1);
const loaded = ref("");

watch(viewPage, (newValue) => {
  if (newValue !== null) {
    window.scrollTo({
      top: 0,
      behavior: "smooth",
    });
    loadGallery(newValue, 1, false).then(() => {
      galleryPage.value = 1;
    });
    const next_id = nextId();
    if (next_id) {
      loadGallery(next_id, 1, true);
    }
  }
});

watch(galleryPage, (newValue) => {
  if (newValue !== null && viewPage.value) {
    loadGallery(viewPage.value, newValue, false);
  }
});

const searchQuery = computed(() => String(route.query.f_search || ""));

const lastId = computed(() =>
  route.query.prev
    ? Number(route.query.prev)
    : route.query.next
      ? Number(route.query.next)
      : null,
);
const forward = computed(() => Boolean(route.query.next) || !route.query.prev);
const cookie = useCookie("EX_COOKIE");

const { data: items } = await useAsyncData(
  () => `search-${searchQuery.value}-${lastId.value}-${forward.value}`,
  async () => {
    try {
      const s = await ses.search(
        searchQuery.value,
        lastId.value !== null ? BigInt(lastId.value) : null,
        forward.value,
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

function makeCacheKey(id: number, page: number): GalleryKey {
  return `${id}:${page}`;
}

async function loadGallery(id: number, page: number, cache: boolean) {
  const key = makeCacheKey(id, page);
  if (cache && galleryCache.value.has(key)) {
    return;
  }
  if (!cache && galleryCache.value.has(key)) {
    galleryData.value = galleryCache.value.get(key);
    galleryCache.value.delete(key);
    return;
  }
  try {
    if (!items.value || "error" in items.value) return;
    const item = items.value.items.find((v) => v.id == id);

    if (!item) return;
    if (!cache && loaded.value == key) {
      return;
    }
    const d = await ses.info(BigInt(id), item.token, page);

    const cookie_val = await ses.cookie();
    if (cookie_val != cookie.value) cookie.value = cookie_val;
    if (!cache) {
      loaded.value = key;
      galleryData.value = d;
    } else {
      galleryCache.value.set(key, d);
    }
  } catch (e) {
    if (!cache) {
      galleryData.value = { error: e };
    }
  }
}

const resultsFiltered = computed(() => {
  if (!items.value) return [];
  let a: SearchInfo[] = items.value.items;
  if (!showFav.value) {
    a = items.value.items.filter(
      (item) => item.favorite == undefined && !item.voted,
    );
  }
  if (!showUser.value) {
    a = a.filter((item) => item.publisher !== "username2983740");
  }
  return a;
});

function setNextId() {
  const id = nextId();
  if (id) {
    viewPage.value = id;
  } else {
    viewPage.value = null;
    galleryData.value = null;
    if (items.value && !("error" in items.value)) {
      navigateTo({
        path: route.path,
        query: {
          ...route.query,
          prev: undefined,
          next: items.value.items[items.value.items.length - 1].id,
        },
      });
    }
  }
}

function nextId() {
  const index = resultsFiltered.value.findIndex((v) => v.id == viewPage.value);
  if (index == -1 || index >= resultsFiltered.value.length - 1) {
    return null;
  }
  return resultsFiltered.value[index + 1].id;
}
</script>

<template>
  <div>
    <NavBar />
    <div v-if="items && items.error">{{ items.error }}</div>
    <template v-if="viewPage">
      <InfoComponent
        v-if="galleryData && !('error' in galleryData)"
        v-model:page="galleryPage"
        :gallery-data="galleryData"
        @close="() => setNextId()"
      />
      <div v-else>{{ galleryData?.error }}</div>
    </template>
    <template v-else>
      <div v-if="items && items.items && items.items.length === 0">
        No items found
      </div>
      <div
        v-if="items && items.items && items.items.length > 0 && !items.error"
        class="ido"
        :style="view == 't' ? 'max-width: var(--container-width)' : ''"
      >
        <h1 class="ih">
          E-Hentai Galleries: The Free Hentai Doujinshi, Manga and Image Gallery
          System
        </h1>
        <SearchArea v-model:showuser="showUser" v-model:showfav="showFav" />
        <div
          v-if="items && items.items.length > 0"
          style="position: relative; z-index: 2"
        >
          <RangeBar
            :progress_min="items.progress_min"
            :progress_max="items.progress_max"
          />
          <FoundItems :text="items.count" />
          <CommonView
            :first="items.first"
            :last="items.last ? null : '1'"
            :query="searchQuery"
            :prev="items.first ? null : items.items[0].id"
            :next="items.last ? null : items.items[items.items.length - 1].id"
            :items="resultsFiltered"
            @open-all="resultsFiltered[0] && (viewPage = resultsFiltered[0].id)"
          />
        </div>
      </div>
    </template>
  </div>
</template>

<style lang="css" src="~/styles/index.css" scoped></style>
