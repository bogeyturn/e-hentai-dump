<script setup lang="ts">
import NavBar from "~/components/NavBar.vue";
import { withDefaults } from "#imports";
import FavoritesPage from "~/components/FavoritesPage.vue";

const data = useTypedQuery<{
  f_search: string;
  next: string;
  prev: string;
  favcat?: number;
}>({
  f_search: withDefaults(string(), ""),
  next: withDefaults(string(), ""),
  prev: withDefaults(string(), ""),
  favcat: optional(number()),
});

const order = useCookie("fav-order", {
  default: () => "p",
});
</script>
<template>
  <div>
    <NavBar />
    <div v-if="data.error">{{ data.error }}</div>
    <FavoritesPage
      v-if="data.data"
      :next="data.data.next"
      :prev="data.data.prev"
      :query="data.data.f_search"
      :order-by-published="order == 'p'"
    />
  </div>
</template>
