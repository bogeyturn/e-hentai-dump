<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, watch } from "vue";
const route = useRoute();

const id = route.params.id as string;
const token = route.params.token as string;
const ses = getSession();

useHead({
  bodyAttrs: {
    style: "overflow: hidden;",
  },
});

async function load_img_info(idx: number, key: string) {
  return await session.nextImg(BigInt(parseInt(id)), key, idx);
}

interface ImageItem {
  id: number;
  name: string;
  key: string;
  src: string;
  width: number;
  height: number;
  loading?: boolean;
  img_info?: {
    url: string;
    original?: string;
    size: string;
    name: string;
    hash: string;
    w: number;
    h: number;
  };
}

async function ensureImgInfo(n: ImageItem) {
  if (n.loading || n.img_info) return;

  n.loading = true;
  try {
    const info = await load_img_info(n.id, n.key);
    n.img_info = info;
  } finally {
    n.loading = false;
  }
}

const currentPage = ref(1);
const imageEls = ref<HTMLElement[]>([]);

const container = ref<HTMLElement | null>(null);

let observer: IntersectionObserver | null = null;

function preloadWindow(centerIndex: number) {
  const start = centerIndex;
  const end = Math.min(images.length - 1, centerIndex + 5);

  for (let i = start; i <= end; i++) {
    ensureImgInfo(images[i]!);
  }
}

onMounted(() => {
  observer = new IntersectionObserver(
    (entries) => {
      for (const entry of entries) {
        if (!entry.isIntersecting) continue;

        const index = Number(entry.target.id.replace("image_", "")) - 1;

        preloadWindow(index);
      }
    },
    {
      root: null,
      rootMargin: "1500px 0px",
      threshold: 0.01,
    },
  );

  imageEls.value.forEach((el) => observer!.observe(el));
});

onBeforeUnmount(() => {
  observer?.disconnect();
});

function scrollToPage(id: number) {
  document.getElementById(`page-${id}`)?.scrollIntoView({
    behavior: "smooth",
    block: "start",
  });
}

function onKey(e: KeyboardEvent) {
  if (e.altKey || e.ctrlKey || e.metaKey) return;

  switch (e.key) {
    case "ArrowRight":
    case "ArrowDown":
    case "d":
      currentPage.value = Math.min(images.length, currentPage.value + 1);
      break;
    case "ArrowLeft":
    case "ArrowUp":
    case "a":
      currentPage.value = Math.max(1, currentPage.value - 1);
      break;
    case " ":
      container.value?.scrollBy({ top: window.innerHeight * 0.75 });
      e.preventDefault();
      break;
  }
}

function enterFullscreen() {
  document.documentElement.requestFullscreen?.();
}

function setImageEl(el: HTMLElement | null) {
  if (!el) return;
  imageEls.value.push(el);
}

onBeforeUpdate(() => {
  imageEls.value = [];
});

onMounted(() => {
  window.addEventListener("keydown", onKey);
});

onBeforeUnmount(() => {
  window.removeEventListener("keydown", onKey);
});

watch(currentPage, scrollToPage);
</script>

<template>
  <div id="pane_outer">
    <div id="bar1">
      <div id="bar2">
        <div id="bar3">
          <picture>
            <source
              srcset="/g/xmpvc.png"
              media="(prefers-color-scheme: dark)"
            />
            <img
              src="/g/mpvc.png"
              onclick="action_gallery()"
              title="Close Image Viewer"
              style="margin: 3px"
            />
          </picture>
          <picture>
            <source
              srcset="/g/xmpvf.png"
              media="(prefers-color-scheme: dark)"
            />
            <img
              src="/g/mpvf.png"
              onclick="do_fullscreen()"
              style="margin-top: 20px"
              title="Go Fullscreen - F11 or ESC to cancel"
            />
          </picture>
          <picture>
            <source
              srcset="/g/xmpvsn.png"
              media="(prefers-color-scheme: dark)"
            />
            <img
              src="/g/mpvsn.png"
              onclick="action_set('ms_n')"
              style="margin-top: 20px; opacity: 0.5"
              title="Align Left, Scale Down Only"
            />
          </picture>
          <picture>
            <source
              srcset="/g/xmpvsc.png"
              media="(prefers-color-scheme: dark)"
            />
            <img
              src="/g/mpvsc.png"
              onclick="action_set('ms_c')"
              style="margin-top: 5px"
              title="Align Center, Scale Down Only"
            />
          </picture>
          <picture>
            <source
              srcset="/g/xmpvsy.png"
              media="(prefers-color-scheme: dark)"
            />
            <img
              src="/g/mpvsy.png"
              onclick="action_set('ms_y')"
              style="margin-top: 5px; opacity: 0.5"
              title="Align Center, Scale To Fit"
            />
          </picture>
          <picture>
            <source
              srcset="/g/xmpvtn.png"
              media="(prefers-color-scheme: dark)"
            />
            <img
              src="/g/mpvtn.png"
              onclick="action_set('mt_n')"
              style="margin-top: 20px"
              title="Show Thumbnail Pane"
            />
          </picture>
          <picture>
            <source
              srcset="/g/xmpvty.png"
              media="(prefers-color-scheme: dark)"
            />
            <img
              src="/g/mpvty.png"
              onclick="action_set('mt_y')"
              style="margin-top: 5px; opacity: 0.5"
              title="Hide Thumbnail Pane"
            />
          </picture>
        </div>
      </div>
    </div>

    <div id="pane_thumbs">
      <a v-if="true" v-for="n in images" :key="n.id" :href="`#page${n.id}`">
        <div
          :id="`thumb_${n.id}`"
          style="width: 200px; height: 280px; visibility: visible"
          :style="{
            background: `transparent url('${n.src}') 0px 0px no-repeat`,
          }"
        ></div>
      </a>
    </div>

    <div id="pane_images" class="full-wside sidebar">
      <template v-for="n in images" :key="n.id">
        <a :name="`page${n.id}`"> </a>
        <div :id="`image_${n.id}`" class="mimg" :ref="setImageEl">
          <img
            v-if="n.img_info"
            :id="`imgsrc_${n.id}`"
            :src="n.img_info.url"
            :title="n.img_info.name"
            style="margin: 0; width: 700px; width: 100%"
          />
          <div v-if="n.img_info" class="mbar">
            <div v-if="n.img_info.original">
              <NuxtLink
                :to="n.img_info.original"
                :target="`fullimg_${n.id}_${id}`"
                ><picture>
                  <source
                    srcset="/g/xmpvd.png"
                    media="(prefers-color-scheme: dark)"
                  />
                  <img
                    title="Open image in normal viewer"
                    src="/g/mpvd.png"
                  /> </picture
              ></NuxtLink>
            </div>
            <div>
              <picture>
                <source
                  srcset="/g/xmpvr.png"
                  media="(prefers-color-scheme: dark)"
                />
                <img title="Open image in normal viewer" src="/g/mpvr.png" />
              </picture>
            </div>
            <div>
              {{ n.img_info.w }} x {{ n.img_info.h }} ::
              {{ n.img_info.size }} ::
              {{ n.img_info.name }}
            </div>
            <div>
              <NuxtLink
                :to="`/s/${n.key}/${n.id}-${id}`"
                :target="`viewer_${n.id}_${id}`"
                ><picture>
                  <source
                    srcset="/g/xmpvn.png"
                    media="(prefers-color-scheme: dark)"
                  />
                  <img
                    title="Open image in normal viewer"
                    src="/g/mpvn.png"
                  /> </picture
              ></NuxtLink>
            </div>
            <div>
              <NuxtLink
                :to="`/?f_shash=${n.img_info.hash}`"
                :target="`search_${n.id}_${id}`"
              >
                <picture>
                  <source
                    srcset="/g/xmpvs.png"
                    media="(prefers-color-scheme: dark)"
                  />
                  <img title="Open image in normal viewer" src="/g/mpvs.png" />
                </picture>
              </NuxtLink>
            </div>
          </div>
        </div>
      </template>
    </div>
  </div>
</template>

<style scoped>
.mimg {
  width: 100%;
  min-width: 490px;
  padding: 0;
  margin: 0 0 4px;
  background: #f2efdf;
}

@media (prefers-color-scheme: dark) {
  .mimg {
    background: #43464e;
  }
}

.mbar {
  margin: auto;
  display: flex;
  flex-flow: row nowrap;
  justify-content: center;
}

.mbar img {
  margin: 0 2px;
  cursor: pointer;
}

.mbar > div:nth-child(3) {
  flex-grow: 1;
  flex-shrink: 1;
  overflow: hidden;
  white-space: nowrap;
  text-overflow: ellipsis;
  font-size: 10pt;
  padding: 3px;
}

#bar1 {
  height: 1px;
  width: 100%;
}

#bar2 {
  position: relative;
  float: right;
  width: 35px;
  height: 1px;
}

#bar3 {
  position: absolute;
  top: 0px;
  left: 0px;
  width: 35px;
  height: 270px;
  z-index: 3;
  text-align: right;
}

#bar3 img {
  cursor: pointer;
  margin: 3px 5px 3px 3px;
}

#pane_outer {
  width: 100vw;
  height: 100vh;
  margin: 0;
  padding: 0;
  position: relative;
}

#pane_thumbs {
  position: absolute;
  top: 0px;
  left: 0px;
  width: 220px;
  height: 100%;
  overflow-x: hidden;
  overflow-y: scroll;
}

#pane_thumbs div {
  margin: auto;
}

#pane_images {
  position: absolute;
  top: 0px;
  left: 0;
  height: 100%;
  overflow: scroll;
}
#pane_images.sidebar {
  left: 225px;
}
#pane_images.full-wside {
  width: calc(100% - 225px - 45px);
}

#pane_images.full-noside {
  width: calc(100% - 45px);
}
#pane_images.center {
  right: 0;
  transform: translateX(-50%);
}
</style>
