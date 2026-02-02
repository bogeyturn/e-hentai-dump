<script setup lang="ts">
import type { Gallery } from "exx";
import ManageSubList from "~/components/upload/ManageSubList.vue";

defineProps<{
  lists: [string, Gallery[]][];
  published: boolean;
  event: number;
}>();
</script>

<template>
  <template v-for="(items, i) in lists" :key="i">
    <ManageSubList :items="items" :published="published" :event="event" />
  </template>
</template>

<style scoped></style>
