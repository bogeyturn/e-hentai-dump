<script setup lang="ts">
import NavBar from "~/components/NavBar.vue";
const route = useRoute();
const ulgid = computed(() => String(route.query.ulgid || ""));
const gid = computed(() => String(route.query.gid || ""));
</script>

<template>
  <div>
    <NavBar />
    <SubNavBar
      :nav-items="uploadNavItems"
      :bold="!ulgid && !gid ? 'Create New Gallery' : ''"
    />
  </div>
</template>

<style scoped></style>
