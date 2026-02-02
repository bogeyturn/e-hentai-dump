<script setup lang="ts">
import NavBar from "~/components/NavBar.vue";
const ses = getSession();
const cookie = useCookie("EX_COOKIE");

const { data } = await useAsyncData(
  () => `tos`,
  async () => {
    try {
      const v = await ses.tos();
      const cookie_val = await ses.cookie();
      if (cookie_val != cookie.value) cookie.value = cookie_val;
      return v;
    } catch (err) {
      return { error: err };
    }
  },
  { server: true, immediate: true },
);
</script>

<template>
  <div>
    <NavBar />
    <!-- eslint-disable vue/no-v-html -->
    <div
      class="stuffbox"
      style="
        text-align: left;
        width: 980px;
        margin: 10px auto 10px auto;
        padding: 5px;
      "
      v-html="data"
    />
  </div>
</template>
