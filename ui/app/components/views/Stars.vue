<script setup lang="ts">
const props = defineProps<{
  star: number;
  opacity: number;
  color?: "r" | "b" | "g" | "";
}>();

const color = computed(() => {
  if (props.color == "r") {
    return "irr";
  } else if (props.color == "g") {
    return "irg";
  } else if (props.color == "b") {
    return "irb";
  } else {
    return "";
  }
});

const style = computed(
  () =>
    `background-position:${-80 + 16 * Math.floor((props.star + 1) / 2)}px ${-1 - (props.star % 2 == 0 ? 0 : 20)}px;opacity:${props.opacity}`,
);
</script>

<template>
  <div class="ir" :class="color" :style="style" />
</template>

<style scoped></style>
