<script setup lang="ts">
import { wrapUrl } from "#imports";

defineProps<{ url: string; width: number; height: number }>();
</script>

<template>
  <div id="gleft">
    <div id="gd1">
      <div
        :style="`
              width: ${width}px;
              height: ${height}px;
              background: transparent
                url(${wrapUrl(url)}) 0 0
                no-repeat;
            `"
      />
    </div>
  </div>
</template>

<style scoped></style>
