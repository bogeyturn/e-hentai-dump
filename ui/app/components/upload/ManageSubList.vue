<script setup lang="ts">
import type { Gallery } from "exx";

const isOpen = ref(true);
const route = useRoute();

const props = defineProps<{
  items: [string, Gallery[]];
  published: boolean;
  event: number;
}>();

const order = computed(() =>
  String(route.query.ss || "") == "d" ? "date" : "name",
);
const order_asc = computed(() => String(route.query.sd || "") != "d");

const ordered_items = computed(() => {
  const items = [...props.items[1]];
  items.sort((a, b) => {
    let valA: string | number;
    let valB: string | number;

    if (order.value === "name") {
      valA = a.name.toLowerCase();
      valB = b.name.toLowerCase();
    } else if (order.value === "date") {
      valA = a.date.secs;
      valB = b.date.secs;
    } else {
      return 0;
    }

    if (valA < valB) return order_asc.value ? -1 : 1;
    if (valA > valB) return order_asc.value ? 1 : -1;
    return 0;
  });
  return items;
});

const selected = ref<Record<number, boolean>>({});

function selectAll() {
  props.items[1].forEach((item: Gallery) => {
    selected.value[item.uid] = true;
  });
}

function deselectAll() {
  props.items[1].forEach((item: Gallery) => {
    selected.value[item.uid] = false;
  });
}

function isSelected(key: number) {
  return !!selected.value[key];
}

function toggleSelected(key: number) {
  selected.value[key] = !selected.value[key];
}

const lastEvent = ref<number>(0);

watch(
  () => props.event,
  (newEvent) => {
    if (newEvent !== lastEvent.value) {
      lastEvent.value = newEvent;
      isOpen.value = false;
    }
  },
);
</script>

<template>
  <template v-if="true">
    <tr class="gtr">
      <td colspan="4" class="l">
        <a class="fview_u" href="#" @click="isOpen = !isOpen"
          >[{{ isOpen ? "-" : "+" }}]</a
        >&nbsp; <strong>{{ items[1].length }}</strong
        >&nbsp; <span>{{ items[0] }}</span>
      </td>
      <td colspan="2" class="r">
        <span @click="selectAll()">+ All</span>
        <span @click="deselectAll()">- All</span>
      </td>
    </tr>
    <tr v-if="isOpen" v-for="(item, j) in ordered_items" :key="j">
      <td class="gtc1">
        <NuxtLink :to="`/upload/managegallery?ulgid=${item.uid}`">{{
          item.name
        }}</NuxtLink>
      </td>
      <td class="gtc2">{{ displayDate(item.date) }}</td>
      <td class="gtc3">{{ item.files }}</td>
      <td class="gtc4" v-if="item.category == 'Expunged'">
        <NuxtLink class="e" :to="`/g/${item.uid}/${item.uid}/?act=expunge`"
          >Expunged</NuxtLink
        >
      </td>
      <td v-else class="gtc4">{{ item.category }}</td>
      <td v-if="!published" class="gtc5">
        [
        <NuxtLink :to="`/upload/managegallery?ulgid=${item.uid}`"
          >Manage</NuxtLink
        >
        ] &nbsp;[<a>Delete</a>] &nbsp;[<a>Publish</a>]
      </td>
      <td v-if="published" class="gtc5">
        [
        <NuxtLink :to="`/g/${item.uid}/${item.uid}/`" target="_ost"
          >View</NuxtLink
        >
        ] &nbsp;[
        <NuxtLink :to="`/stats?gid=${item.uid}&amp;t=${item.uid}`"
          >Stats
        </NuxtLink>
        ] &nbsp;[
        <NuxtLink :to="`/upload/managegallery?gid=${item.uid}`"
          >Manage
        </NuxtLink>
        ] &nbsp;[
        <NuxtLink>Disown </NuxtLink>
        ]
      </td>
      <td class="gtc6">
        <input
          type="checkbox"
          class="check_u0"
          @click="toggleSelected(item.uid)"
          :checked="isSelected(item.uid)"
        />
      </td>
    </tr>
  </template>
</template>

<style scoped>
body {
  font-size: 9pt;
}

a {
  text-decoration: none;
}

h2 {
  font-weight: bold;
  font-size: 10pt;
}

table#t td {
  text-align: right;
  margin-top: 5px;
  padding-right: 10px;
}

table#t select {
  width: 200px;
}

p#c a {
  font-weight: bold;
}

div.l {
  float: left;
}

div.r {
  float: right;
  padding-right: 5px;
}

th {
  padding: 2px;
}
a.e {
  font-weight: bold;
  color: red;
  text-decoration: underline;
}

td.l {
  height: 22px;
  padding-top: 1px;
  padding-left: 5px;
  text-align: left;
  font-size: 10pt;
}

td.l span {
  font-style: italic;
  font-weight: bold;
}

td.l a {
  font-family: courier, serif;
  position: relative;
  top: -1px;
}

td.r {
  height: 22px;
  padding-top: 2px;
  padding-left: 5px;
  text-align: right;
}

td.r span {
  margin: 0px 2px;
  vertical-align: middle;
  cursor: pointer;
}

td.gtc1 {
  height: 22px;
  padding-left: 10px;
  text-align: left;
  max-width: 450px;
  overflow: hidden;
  text-overflow: ellipsis;
}

td.gtc3 {
  padding-right: 10px;
  text-align: right;
}

td.gtc4 {
  text-align: center;
}

td.gtc6 {
  text-align: center;
}
</style>
