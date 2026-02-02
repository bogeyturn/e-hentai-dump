<script setup lang="ts">
function openPopup(URL: string, w: number, h: number) {
  window.open(URL, "_pu" + (Math.random() + "").replace(/0\./, ""), "toolbar=0,scrollbars=0,location=0,statusbar=0,menubar=0,resizable=0,width=" + w + ",height=" + h + ",left=" + ((screen.width - w) / 2) + ",top=" + ((screen.height - h) / 2));
  return false;
}

const BASE_URL = "https://exhentai.org/";

const emit = defineEmits(["report", "explunge"])
const props = defineProps<{ gid: bigint, token: string }>()
const route = useRoute()
const rows = computed(() => [
  {
    key: "report",
    class: "g3 gsp",
    img: "https://exhentai.org/img/mr.gif",
    label: "Report Gallery",
    src: {query: {report: "select"}}
  },
  {
    key: "archive",
    class: "g2 gsp",
    label: "Archive Download",
    action: () =>
        openPopup(
            `${BASE_URL}archiver.php?gid=${props.gid}&token=${props.token}`,
            480,
            320,
        ),
  },
  {
    key: "torrent",
    class: "g2",
    label: "Torrent Download (0)",
    action: () =>
        openPopup(
            `${BASE_URL}gallerytorrents.php?gid=${props.gid}&t=${props.token}`,
            610,
            590,
        ),
  },
  {
    key: "explunge",
    class: "g2 gsp",
    label: "Petition to Expunge",
    src: {query: {act: "explunge"}}
  },
  {
    key: "rename",
    class: "g2",
    label: "Petition to Rename",
    action: () =>
        openPopup(
            `${BASE_URL}gallerypopups.php?gid=${props.gid}&t=${props.token}&act=rename`,
            675,
            415,
        ),
  },
  {
    key: "mpv",
    class: "g2 gsp",
    label: "Multi-Page Viewer",
    src: `/mpv/${props.gid}/${props.token}/`
  },
]);

</script>

<template>
  <div id="gd5">
    <p
        v-for="row in rows"
        :key="row.key"
        :class="row.class"
    >
      <img src="/img/mr.gif" alt=""/>
      <a v-if="row.src==undefined" href="#" @click.prevent="row.action">
        {{ row.label }}
      </a>
      <NuxtLink v-else :to="row.src"> {{ row.label }} </NuxtLink>
    </p>
  </div>
</template>

<style scoped></style>
