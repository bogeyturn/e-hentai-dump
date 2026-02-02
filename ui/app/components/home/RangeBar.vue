<template>
  <div id="rangebar" style="height: 1rem; position: relative">
    <div
      style="
        background-color: #3b82f6;
        height: 100%;
        border-radius: 0.5rem;
        position: absolute;
        transition-property: all;
        transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
        transition-duration: 500ms;
      "
      :style="{
        left: barLeft,
        width: barWidth,
      }"
    ></div>
  </div>
</template>

<script setup>
import { computed } from "vue";

const props = defineProps({
  progress_min: { type: Number, required: true },
  progress_max: { type: Number, required: true },
});

const barLeft = computed(() => {
  if (props.progress_max - props.progress_min <= 0.005) return "0%";
  return (props.progress_min * 100).toFixed(0) + "%";
});

const barWidth = computed(() => {
  if (props.progress_max - props.progress_min <= 0.005) {
    return props.progress_max * 100 + "%";
  }
  return (props.progress_max - props.progress_min) * 100 + "%";
});
</script>
