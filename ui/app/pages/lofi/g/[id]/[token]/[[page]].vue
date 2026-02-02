<script setup lang="ts">
import { useAsyncData } from "#imports";

const route = useRoute();
const id = route.params.id as string;
const token = route.params.token as string;
const page = route.params.page
  ? String(route.params.page.length > 0 ? route.params.page : "0")
  : "";
const ses = getSession();

const cookie = useCookie("EX_COOKIE");
const { data: galleryData } = useAsyncData(
  () => `gallery-${id}-${page}`,
  async () => {
    const parsed_id = Number(id);
    if (!token || isNaN(parsed_id) || isNaN(Number(page))) return null;
    const v = await ses.info(BigInt(parsed_id), token, parseInt(page) + 1);
    const cookie_val = await ses.cookie();
    if (cookie_val != cookie.value) cookie.value = cookie_val;
    return v;
  },
  { server: true, immediate: true },
);
</script>

<template>
  <div v-if="galleryData">
    <h1>
      <NuxtLink :to="`/lofi/g/${id}/${token}/`"
        >{{ galleryData.title }}
      </NuxtLink>
    </h1>
    <div id="gh">
      <NuxtLink
        v-for="p in galleryData.pages"
        :key="p.id"
        :to="`/lofi/s/${p.key}/${id}-${p.id}`"
      >
        <div
          :title="p.name"
          :style="`width:200px;aspect-ratio:${p.ratio[0]} / ${p.ratio[1]};background:${p.url}`"
        />
      </NuxtLink>
    </div>
    <div id="ia">
      <span v-if="page == '0'">&lt; Prev Page</span>
      <NuxtLink v-else :to="`/lofi/g/${id}/${token}/${parseInt(page) - 1}`"
        >&lt; Prev Page</NuxtLink
      >
      &nbsp; &nbsp;
      <NuxtLink to="/lofi/">Back</NuxtLink> &nbsp; &nbsp;
      <span v-if="galleryData.files / 20 <= parseInt(page) + 1"
        >Next Page &gt;</span
      >
      <NuxtLink v-else :to="`/lofi/g/${id}/${token}/${parseInt(page) + 1}`"
        >Next Page &gt;
      </NuxtLink>
    </div>
  </div>
</template>

<style scoped></style>
