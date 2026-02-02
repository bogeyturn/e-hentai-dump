<script setup lang="ts">
import { computed, wrapUrl } from "#imports";

const route = useRoute();
const ses = getSession();

const searchQuery = computed(() => String(route.query.f_search || ""));

const lastId = computed(() =>
  route.query.prev
    ? Number(route.query.prev)
    : route.query.next
      ? Number(route.query.next)
      : null,
);
const forward = computed(() => Boolean(route.query.next) || !route.query.prev);
const new_search = ref(searchQuery.value);
const cookie = useCookie("EX_COOKIE");
const { data: items } = await useAsyncData(
  () => `search-${searchQuery.value}-${lastId.value}-${forward.value}`,
  async () => {
    try {
      const v = await ses.search(
        searchQuery.value,
        lastId.value !== null ? BigInt(lastId.value) : null,
        forward.value,
      );
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
    <h1><NuxtLink to="/lofi/">E-Hentai Lo-Fi Galleries</NuxtLink></h1>
    <form @submit.prevent="">
      <div id="is">
        <input
          id="search"
          v-model="new_search"
          type="text"
          name="f_search"
          size="45"
          maxlength="200"
          placeholder="Search Keywords"
        />
        <NuxtLink
          :to="{
            path: route.path,
            query: {
              ...route.query,
              prev: undefined,
              next: undefined,
              f_search: new_search,
            },
          }"
          ><input type="submit" value="Search" /><input
            type="button"
            value="Clear"
        /></NuxtLink>
      </div>
    </form>

    <div v-if="items && !('error' in items)" id="ig">
      <div v-for="(item, index) in items.items" :key="index">
        <div>
          <NuxtLink :to="`/lofi/s/${0}/${item.id}-1`"
            ><img :src="wrapUrl(item.img ?? '')" alt="Cover Image"
          /></NuxtLink>
        </div>
        <div>
          <h2>
            <NuxtLink :to="`/lofi/g/${item.id}/${item.token}/`"
              >{{ item.title }}
            </NuxtLink>
          </h2>
          <p>
            {{ item.published }} &nbsp; &nbsp; {{ item.category }} &nbsp; &nbsp;
            {{ item.pages }}p &nbsp; &nbsp;
            {{ item.publisher }}
          </p>
          <table class="tl">
            <tbody>
              <tr>
                <td>other:</td>
                <td>ai generated</td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>
    <div v-if="items && !('error' in items)" id="ia">
      <span v-if="items.first">&lt; Prev Page</span>
      <NuxtLink v-else :to="`/lofi/?prev=${items.items[0]?.id}`"
        >&lt; Prev Page</NuxtLink
      >
      <span v-if="items.last">Next Page &gt;</span>
      &nbsp; &nbsp;
      <NuxtLink v-if="!items.last" :to="`/lofi/?next=${items.items.at(-1)?.id}`"
        >Next Page &gt;</NuxtLink
      >
    </div>
  </div>
</template>
