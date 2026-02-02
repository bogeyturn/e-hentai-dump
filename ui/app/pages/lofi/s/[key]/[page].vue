<script setup lang="ts">
import { useAsyncData } from "#imports";

const ses = getSession();

const route = useRoute();
const key = route.params.key as string;
const id = computed(() => String(route.params.page).split("-", 2)[0] ?? "");
const page = computed(() => String(route.params.page).split("-", 2)[1] ?? "");

const cookie = useCookie("EX_COOKIE");
const { data: img_data } = useAsyncData(
  () => `image-${key}-${id.value}${page.value}`,
  async () => {
    if (!key || isNaN(Number(id.value)) || isNaN(Number(page.value)))
      return null;
    const v = await ses.nextImg(
      BigInt(parseInt(id.value)),
      key,
      parseInt(page.value),
    );
    const cookie_val = await ses.cookie();
    if (cookie_val != cookie.value) cookie.value = cookie_val;
    return v;
  },
  { server: true, immediate: true },
);
</script>

<template>
  <div v-if="img_data" id="so">
    <div id="sd">
      <NuxtLink :to="`/lofi/s/${key}/${id}-${page}`">
        <img
          id="sm"
          :src="img_data.url"
          :alt="img_data.name"
          :title="img_data.name"
      /></NuxtLink>
    </div>
    <div id="ia">
      <NuxtLink
        v-if="img_data.prev"
        :to="`/lofi/s/${img_data.prev}/${id}-${parseInt(page) - 1}`"
        >&lt; Prev Page</NuxtLink
      >
      <span v-else>&lt; Prev Page</span>

      &nbsp; &nbsp;

      <NuxtLink :to="`/lofi/g/${id}/${img_data.gallery_key}/`">Back </NuxtLink>

      &nbsp; &nbsp;

      <NuxtLink
        v-if="img_data.next"
        :to="`/lofi/s/${img_data.next}/${id}-${parseInt(page) + 1}`"
        >Next Page &gt;
      </NuxtLink>
      <span v-else>Next Page &gt;</span>
    </div>
  </div>
</template>

<style scoped></style>
