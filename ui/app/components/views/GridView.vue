<script setup lang="ts">
import GridItem from "~/components/views/grid/GridItem.vue";
import type {SearchInfo} from "exx";

const scale = useCookie("search-cover-scaling", {
  default: () => 1.0,
});

useHead({
  script: [
    {
      innerHTML: `
        (function() {
          const w = window.innerWidth;
          let col = 0;
          const scale = ${scale.value};
          if (w <= 820*scale) col = 3;
          else if (w <= 1090*scale) col = 2;
          else if (w <= 1360*scale) col = 1;
          document.documentElement.style.setProperty('--cols', col);
          document.documentElement.style.setProperty('--scale', scale);
        })();
      `,
      type: "text/javascript",
    },
  ],
});

defineProps({
  items: {required: true, type: Array<SearchInfo>},
});

const colClass = computed(() => {
  const width = windowWidth.value;
  let col = 0;
  if (width <= 1360 * scale.value) col = 1;
  if (width <= 1090 * scale.value) col = 2;
  if (width <= 820 * scale.value) col = 3;
  if (document && document.documentElement) {
    document.documentElement.style.setProperty("--cols", String(col));
    document.documentElement.style.setProperty("--scale", String(scale.value));
  }
  if (col == 1) return "col4";
  if (col == 2) return "col3";
  if (col == 3) return "col2";
  return "";
});

const windowWidth = ref(0);

function updateWidth() {
  windowWidth.value = window.innerWidth;
}

onMounted(() => {
  updateWidth();
  window.addEventListener("resize", updateWidth);
});

onBeforeUnmount(() => {
  window.removeEventListener("resize", updateWidth);
});
</script>

<template>
  <div class="itg gld" :class="colClass">
    <GridItem
        v-for="item in items"
        :key="item.id"
        :idd="item.id"
        :secret="item.token"
        :title="item.title"
        :cover="item.img ? item.img : ''"
        :category="item.category"
        :voted="item.voted"
        :date="item.published"
        :pages="item.pages"
        :stars="item.rating"
        :disowned="item.disowned"
        :fav="item.favorite"
        :new-item="item.new"
    />
  </div>
</template>

<style lang="css" src="~/styles/index.css" scoped></style>
