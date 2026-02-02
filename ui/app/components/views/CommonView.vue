<script lang="ts" setup>
import Pagination from "~/components/home/Pagination.vue";
import type { SearchInfo } from "~~/pkg";
import GridView from "~/components/views/GridView.vue";
import CompactView from "~/components/views/CompactView.vue";

const props = defineProps({
  orderEnabled: {
    type: Boolean,
    default: false,
  },
  first: {
    type: Boolean,
    required: true,
  },
  last: {
    required: true,
  },
  query: {
    type: String,
    required: true,
  },
  prev: {
    required: true,
  },
  next: {
    required: true,
  },
  items: {
    type: Array<SearchInfo>,
    required: true,
  },
});

function openLinksSlowly() {
  props.items.forEach((item: SearchInfo, i: number) => {
    setTimeout(() => {
      window.open(`/g/${item.id}/${item.token}/`, "_blank");
    }, i * 200);
  });
}

defineEmits(["open-all"]);

const view = useCookie("search-view", {
  default: () => "t",
});
</script>
<template>
  <Pagination
    :first="first"
    :last="last"
    :mode="true"
    :order-enabled="orderEnabled"
    :query="query"
    :prev="prev"
    :next="next"
    @open-all="
      () => {
        $emit('open-all');
        //openLinksSlowly();
      }
    "
  />
  <GridView v-if="view == 't'" :items="items" />
  <CompactView v-if="view == 'l'" :items="items" />

  <Pagination
    :first="first"
    :last="last"
    :mode="false"
    :query="query"
    :prev="prev"
    :next="next"
    @open-all="
      () => {
        $emit('open-all');
        //openLinksSlowly();
      }
    "
  />
</template>
