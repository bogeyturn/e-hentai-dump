<script setup lang="ts">
const ses = getSession();

const route = useRoute();
const router = useRouter();
const page = computed(() => parseInt((route.query.page as string) || "0"));

const props = defineProps({
  tl: { type: String, required: true },
});

function updatePage(newPage) {
  router.replace({
    query: {
      ...route.query,
      page: newPage - 1,
    },
  });
}
const cookie = useCookie("EX_COOKIE");
const { data } = await useAsyncData(
  () => `toplist-user-${page.value}-${props.tl}`,
  async () => {
    try {
      const v = await ses.toplist(parseInt(props.tl), page.value + 1);
      const cookie_val = await ses.cookie();
      if (cookie_val != cookie.value) cookie.value = cookie_val;
      return v;
    } catch (err) {
      return { error: err };
    }
  },
  { server: true, immediate: true },
);

const new_data = computed(() => {
  if (!data.value) return;
  if ("error" in data.value) return;
  function chunkArray<T>(arr: T[], size: number = 2): T[][] {
    const chunks: T[][] = [];
    for (let i = 0; i < arr.length; i += size) {
      chunks.push(arr.slice(i, i + size));
    }
    return chunks;
  }
  return chunkArray(data.value, 2);
});
</script>

<template>
  <div class="ido" style="min-width: 1020px; max-width: 1270px">
    <h1 class="ih">
      <NuxtLink href="/toplist">EHG Toplists </NuxtLink> &gt;
      <a>Uploader Past Year</a>
    </h1>
    <ImagePagination
      :current-page="page + 1"
      :total-items="10"
      :page-size="1"
      :show-counter="false"
      :top="true"
      @update:page="updatePage"
    />
    <table class="itg" style="width: auto">
      <tbody>
        <tr>
          <th class="hr">Rank</th>
          <th class="hs">Score</th>
          <th class="hn">Name</th>
          <th class="hr">Rank</th>
          <th class="hs">Score</th>
          <th class="hn">Name</th>
        </tr>
        <tr v-for="(items, i) in new_data" :key="i">
          <template v-for="(item, j) in items" :key="j">
            <template v-if="item">
              <td class="dr">{{ item.Rank }}</td>
              <td class="ds">{{ item.Score }}</td>
              <td class="du">
                <NuxtLink :href="`/uploader/${item.Name}`">{{
                  item.Name
                }}</NuxtLink>
              </td>
            </template>
          </template>
        </tr>
      </tbody>
    </table>
    <ImagePagination
      :current-page="page + 1"
      :total-items="10"
      :page-size="1"
      :show-counter="false"
      :top="false"
    />
  </div>
</template>

<style scoped>
h1 {
  font-size: 10pt;
  font-weight: bold;
  margin: 2px auto;
  text-align: center;
  padding-bottom: 6px;
}

h1 a {
  text-decoration: none;
}

h2 {
  font-size: 10pt;
  font-weight: bold;
  margin: 1px;
  padding: 4px 1px 1px;
}

table.itg th {
  font-weight: bold;
  background: #e0ded3;
}

table.itg td {
  border-right: 1px solid #d9d7cc;
}

table.gltc td:first-child {
  text-align: center;
  width: 70px;
}

table.gltc td:first-child p:first-child {
  font-size: 10pt;
  font-weight: bold;
}

td.dr {
  font-weight: bold;
  text-align: right;
  padding-right: 5px;
}

td.ds {
  font-weight: bold;
  text-align: right;
  padding: 2px 5px 2px 2px;
}

td.du {
  text-align: left;
  padding: 2px 5px 2px 2px;
}

td.du a {
  text-decoration: none;
}

th.hr {
  width: 35px;
  text-align: right;
  padding: 3px 4px;
}

th.hs {
  width: 60px;
  text-align: right;
  padding: 3px 4px;
}

div.tdo a {
  text-decoration: none;
}
</style>
