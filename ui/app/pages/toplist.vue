<script setup lang="ts">
import NavBar from "~/components/NavBar.vue";
import TopListOverview from "~/components/toplist/TopListOverview.vue";
import { useRoute } from "#imports";
import TopListUser from "~/components/toplist/TopListUser.vue";

const route = useRoute();

const tl = computed(() => String(route.query.tl || ""));
</script>
<template>
  <div>
    <NavBar />
    <TopListOverview v-if="!tl" />
    <TopListUser v-else :tl="tl" />
  </div>
</template>
