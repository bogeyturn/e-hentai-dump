<script setup lang="ts">
import { computed } from "vue";

const props = defineProps({
  category: {
    type: String,
    required: true,
  },
});
const category_id = computed(() => {
  const cat = categories.find((c) => c.label === props.category);
  return cat ? cat.class : "Unknown";
});
const categories = [
  { id: 2, label: "Doujinshi", class: "ct2" },
  { id: 4, label: "Manga", class: "ct3" },
  { id: 8, label: "Artist CG", class: "ct4" },
  { id: 16, label: "Game CG", class: "ct5" },
  { id: 512, label: "Western", class: "cta" },
  { id: 256, label: "Non-H", class: "ct9" },
  { id: 32, label: "Image Set", class: "ct6" },
  { id: 64, label: "Cosplay", class: "ct7" },
  { id: 128, label: "Asian Porn", class: "ct8" },
  { id: 1, label: "Misc", class: "ct1" },
];
</script>

<template>
  <div :class="category_id">{{ category }}</div>
</template>

<style scoped></style>
