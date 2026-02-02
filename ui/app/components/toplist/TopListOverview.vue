<script setup lang="ts">
import type { ToplistLink } from "~~/pkg";
const ses = getSession();

const cookie = useCookie("EX_COOKIE");
const { data } = await useAsyncData(
  () => `toplist-overview`,
  async () => {
    try {
      const v = await ses.toplists();
      const cookie_val = await ses.cookie();
      if (cookie_val != cookie.value) cookie.value = cookie_val;
      return v;
    } catch (err) {
      return { error: err };
    }
  },
  { server: true, immediate: true },
);

function getToplistLink(link: ToplistLink): string {
  if ("User" in link) {
    return `/uploader/${link.User}`;
  } else if ("Gallery" in link) {
    const [id, name] = link.Gallery;
    return `/g/${id}/${name}`;
  }
  throw new Error("Invalid ToplistLink");
}
</script>

<template>
  <div v-if="data && !('error' in data)" class="ido">
    <h1><a>EHG Toplists</a></h1>
    <template v-for="(item, i) in data" :key="i">
      <hr v-if="i > 0" />
      <div :style="{ margin: i === 0 ? '0 auto 0' : '10px auto 0' }">
        <h2>{{ item.name }}</h2>

        <div v-for="(toplists, j) in item.toplists" :key="j" class="tdo">
          <p>
            <img src="/g/mr.gif" class="mr" alt="&gt;" />
            <NuxtLink
              :to="`/toplist?tl=${toplists.id}`"
              style="font-weight: bold"
              >{{ toplists.name }}
            </NuxtLink>
          </p>
          <table>
            <tbody>
              <tr v-for="(it, k) in toplists.items" :key="k">
                <td class="pso">#{{ it.idx }}</td>
                <td>
                  <div class="tun">
                    <NuxtLink :to="getToplistLink(it.link)">{{
                      it.name
                    }}</NuxtLink>
                  </div>
                </td>
              </tr>
            </tbody>
          </table>
        </div>

        <div class="c" />
      </div>
    </template>
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

td.du a {
  text-decoration: none;
}

div.tdo {
  float: left;
  margin: 4px;
  padding: 2px;
  width: 285px;
  text-align: left;
  font-size: 9pt;
}

div.tdo a {
  text-decoration: none;
}

div.tun {
  overflow: hidden;
  width: 255px;
  height: 15px;
  padding-left: 1px;
}

td.pso {
  font-weight: bold;
  text-align: right;
}
</style>
