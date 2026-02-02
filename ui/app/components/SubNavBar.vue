<script setup lang="ts">
import type { NavItem } from "~/utils/config";

const props = defineProps<{
  bold?: string[] | string;
  navItems: NavItem[];
}>();

const boldSet = computed(() => {
  if (!props.bold) return new Set<string>();
  return new Set(Array.isArray(props.bold) ? props.bold : [props.bold]);
});
</script>

<template>
  <div id="lb">
    <div v-for="item in navItems" :key="item.to">
      [
      <NuxtLink :to="item.to" :class="{ bold: boldSet.has(item.label) }">
        {{ item.label }}
      </NuxtLink>
      ]
    </div>
  </div>
</template>

<style scoped>
.bold {
  font-weight: bold;
}
</style>
