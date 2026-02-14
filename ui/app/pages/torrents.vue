<script setup lang="ts">
import NavBar from "~/components/NavBar.vue";
import TorrentPage from "~/components/TorrentPage.vue";
import {withDefaults} from "~/composables/useTypedQuery";


const data = useTypedQuery<{
  search: string;
  u?: number;
  s: "seeded" | "unseeded" | "";
  o: string;
  page: number;
}>({
  search: withDefaults(string(), ""),
  u: optional(number()),
  s: withDefaults(enumOf(["seeded", "", "unseeded"] as const), ""),
  o: withDefaults(string(), ""),
  page: withDefaults(number(), 0),
});
</script>
<template>
  <div>
    <NavBar/>
    <div v-if="data.error">{{ data.error }}</div>
    <TorrentPage
        v-if="data.data"
        :search="data.data.search"
        :page="data.data.page+1"
        :o="data.data.o"
        :s="data.data.s"
        :u="data.data.u"
    />
  </div>
</template>
