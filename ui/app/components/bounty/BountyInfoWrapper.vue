<script setup lang="ts">
const ses = getSession();
const props = defineProps<{ bid: bigint }>();
const cookie = useCookie("EX_COOKIE");
const { data } = await useAsyncData(
  () => `bountyInfo-${props.bid}`,
  async () => {
    try {
      const v = await ses.bounty_info(props.bid);
      const cookie_val = await ses.cookie();
      if (cookie_val != cookie.value) cookie.value = cookie_val;
      return v;
    } catch (err) {
      return { error: String(err) };
    }
  },
  { server: true, immediate: true },
);
</script>

<template>
  <div v-if="!data || 'error' in data">{{ data?.error }}</div>
  <BountyInfo v-else :data="data" />
</template>

<style scoped></style>
