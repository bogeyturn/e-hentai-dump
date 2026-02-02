<script setup lang="ts">
import ManageSubLists from "~/components/upload/ManageSubLists.vue";
import type { Gallery } from "exx";

defineProps<{ unpublished: boolean; lists: [string, Gallery[]][] }>();

const event = ref(0);

const route = useRoute();
const order = computed(() =>
  String(route.query.ss || "") == "d" ? "date" : "name",
);
const order_asc = computed(() => String(route.query.sd || "") != "d");
</script>

<template>
  <div class="s">
    <div class="h">
      <div class="l">
        {{ unpublished ? "Unpublished" : "Published" }} Galleries
      </div>
      <div class="r">[<a @click="event += 1">Collapse Open Folders</a>]</div>
      <div class="c" />
    </div>
    <table id="gtableu" class="mt">
      <tbody>
        <tr>
          <th class="h1">
            Gallery Name
            <NuxtLink
              :to="{
                path: route.path,
                query: { ...route.query, ss: 'n', sd: 'a' },
              }"
              ><img
                src="/g/menu_down.gif"
                :class="order == 'name' && order_asc ? 's' : 'u'"
                alt="A"
              />
            </NuxtLink>
            <NuxtLink
              :to="{
                path: route.path,
                query: { ...route.query, ss: 'n', sd: 'd' },
              }"
              ><img
                src="/g/menu_up.gif"
                :class="order == 'name' && !order_asc ? 's' : 'u'"
                alt="D"
            /></NuxtLink>
          </th>
          <th class="h2">
            Date Added
            <NuxtLink
              :to="{
                path: route.path,
                query: { ...route.query, ss: 'd', sd: 'a' },
              }"
              ><img
                src="/g/menu_down.gif"
                :class="order == 'date' && order_asc ? 's' : 'u'"
                alt="A"
              />
            </NuxtLink>
            <NuxtLink
              :to="{
                path: route.path,
                query: { ...route.query, ss: 'd', sd: 'd' },
              }"
              ><img
                src="/g/menu_up.gif"
                :class="order == 'date' && !order_asc ? 's' : 'u'"
                alt="D"
            /></NuxtLink>
          </th>
          <th class="h3">Files</th>
          <th class="h4">Public Category</th>
          <th class="h5">Available Actions</th>
          <th class="h6" />
        </tr>
        <ManageSubLists
          :lists="lists"
          :published="!unpublished"
          :event="event"
        />
      </tbody>
    </table>
  </div>
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

div.s {
  padding-top: 3px;
}

div.h {
  padding: 5px 2px;
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

th.h1 {
  text-align: left;
  width: 460px;
  padding-left: 4px;
}

th.h2 {
  text-align: left;
  width: 105px;
}

th.h3 {
  text-align: right;
  width: 40px;
  padding-right: 10px;
}

th.h4 {
  text-align: center;
  width: 110px;
}

th.h5 {
  text-align: left;
  width: 230px;
}

th.h6 {
  text-align: left;
  width: 20px;
}

img.s {
  padding: 2px;
  margin-top: 2px;
  border: 1px dotted #5c0d12;
}

img.u {
  padding: 2px;
  margin-top: 2px;
  border: 1px dotted #e0ded3;
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
  margin: 0 2px;
  vertical-align: middle;
  cursor: pointer;
}
</style>
