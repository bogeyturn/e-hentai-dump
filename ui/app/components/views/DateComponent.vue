<script setup lang="ts">
defineProps<{date: {secs: number}, new: boolean, disowned: boolean, fav?: number}>();

function hexToRgb(fav: number) {
  let hex = "#000";
  switch (fav) {
    case 0:
      hex = "#000";
      break;
    case 1:
      hex = "#f00";
      break;
    case 2:
      hex = "#fa0";
      break;
    case 3:
      hex = "#dd0";
      break;
    case 4:
      hex = "#080";
      break;
    case 5:
      hex = "#9f4";
      break;
    case 6:
      hex = "#4bf";
      break;
    case 7:
      hex = "#00f";
      break;
    case 8:
      hex = "#508";
      break;
    case 9:
      hex = "#e8e";
      break;
  }
  hex = hex.replace(/^#/, "");

  if (hex.length === 3) {
    hex = hex
      .split("")
      .map((c) => c + c)
      .join("");
  }

  if (hex.length !== 6) {
    throw new Error("Invalid hex color");
  }

  const r = parseInt(hex.slice(0, 2), 16);
  const g = parseInt(hex.slice(2, 4), 16);
  const b = parseInt(hex.slice(4, 6), 16);

  return { r, g, b };
}

function style_str(hex: number | null | undefined) {
  if (!hex && hex !== 0) return "";
  const rgb = hexToRgb(hex);
  return `border-color:${hex};background-color:rgba(${rgb.r},${rgb.g},${rgb.b},.1)`;
}
</script>

<template>
  <div :style="style_str(fav)" :class="{ new: 'glnew' }">
    <b v-if="disowned">{{ displayDate(date) }}</b>
    <template v-else>{{ displayDate(date) }}</template>
  </div>
</template>

<style scoped></style>
