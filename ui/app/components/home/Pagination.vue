<template>
  <div class="searchnav">
    <div v-if="orderEnabled">
      Order:
      <select v-model="order">
        <option value="p">Published Time</option>
        <option value="f">Favorited Time</option>
      </select>
    </div>
    <div v-else />

    <div>
      <NuxtLink
        v-if="!first"
        :to="{
          path: route.path,
          query: { ...route.query, prev: undefined, next: undefined },
        }"
        >&lt;&lt; First
      </NuxtLink>
      <span v-if="first">&lt;&lt; First</span>
    </div>
    <div>
      <NuxtLink
        v-if="prev"
        :to="{
          path: route.path,
          query: { ...route.query, prev: prev, next: undefined },
        }"
        >&lt; Prev
      </NuxtLink>
      <span v-if="!prev">&lt; Prev</span>
    </div>
    <div class="jumpbox">
      <a @click="$emit('open-all')">Open All</a>

      <!-- <a @click="$emit('jump')">Jump/Seek</a> -->
    </div>
    <div>
      <NuxtLink
        v-if="next"
        :to="{
          path: route.path,
          query: { ...route.query, next: next, prev: undefined },
        }"
        >Next &gt;
      </NuxtLink>
      <span v-if="!next">Next &gt;</span>
    </div>
    <div>
      <NuxtLink
        v-if="last"
        :to="{
          path: route.path,
          query: { ...route.query, prev: last, next: undefined },
        }"
        >Last &gt;&gt;
      </NuxtLink>
      <span v-if="!last">Last &gt;&gt;</span>
    </div>
    <div>
      <select v-if="mode" v-model="view">
        <option
          v-for="option in options"
          :key="option.value"
          :value="option.value"
        >
          {{ option.label }}
        </option>
      </select>
    </div>
  </div>
</template>

<script setup>
defineEmits(["jump", "open-all"]);
defineProps({
  mode: {
    type: Boolean,
    default: false,
  },
  orderEnabled: {
    type: Boolean,
    default: false,
  },
  next: {
    required: true,
  },
  prev: {
    required: true,
  },
  first: {
    type: Boolean,
    default: false,
  },
  last: {
    default: false,
  },
  query: {
    type: String,
    required: true,
  },
  options: {
    type: Array,
    default: () => [
      { value: "m", label: "Minimal" },
      { value: "p", label: "Minimal+" },
      { value: "l", label: "Compact" },
      { value: "e", label: "Extended" },
      { value: "t", label: "Thumbnail" },
    ],
  },
});

const route = useRoute();
const view = useCookie("search-view", {
  default: () => "t",
});

const order = useCookie("fav-order", {
  default: () => "p",
});
const router = useRouter();
watch(order, () => {
  const query = { ...route.query, next: undefined, prev: undefined };

  router.replace({ query });
});
</script>

<style scoped>
a {
  cursor: pointer;
}
</style>
