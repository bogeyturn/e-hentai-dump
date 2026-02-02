<script lang="ts" setup>
import NavBar from "~/components/NavBar.vue";

import { ref, computed } from "vue";
import { useRoute, useAsyncData } from "#imports";
import InfoComponent from "~/components/info/InfoComponent.vue";

const ses = getSession();

const route = useRoute();

const cookie = useCookie("EX_COOKIE");

const stars = ref(0.0);

const id = route.params.id as string;
const token = route.params.token as string;
const act = computed(() =>String(route.query.act ?? ""));
const report = computed(() =>String(route.query.report ?? ""));

const parsed_id = computed(() => Number(id));

const page = ref(1);

const { data: galleryData } = useAsyncData(
  () => `gallery-${parsed_id.value}-${page.value}`,
  async () => {
    if (!token || isNaN(parsed_id.value)) return null;
    try {
      const d = await ses.info(BigInt(parsed_id.value), token, page.value);
      const cookie_val = await ses.cookie();
      if (cookie_val != cookie.value) cookie.value = cookie_val;
      if (d?.my_stars) {
        stars.value = d.my_stars / 2;
      }
      return d;
    } catch (e) {
      return { error: e };
    }
  },
  { watch: [page] },
);
</script>
<template>
  <div>
    <NavBar />
    <InfoComponent
      v-if="galleryData && !('error' in galleryData)"
      v-model:page="page"
      :gallery-data="galleryData"
      :explunge="act=='explunge'"
      :report="!!report"
      @close="window.open('', '_self').close()"
    />
    <div v-else>{{ galleryData?.error }}</div>
  </div>
</template>
