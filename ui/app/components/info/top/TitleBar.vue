<script setup lang="ts">
defineProps<{ title: string; altTitle: string | null }>();
</script>

<template>
  <div id="gd2">
    <h1 id="gn">
      {{ title }}
    </h1>
    <h1 id="gj">
      {{ altTitle }}
    </h1>
  </div>
</template>

<style scoped></style>
