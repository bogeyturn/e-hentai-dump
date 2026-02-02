<script setup lang="ts">
import { configNavItems } from "~/utils/config";
import NavBar from "~/components/NavBar.vue";
import SubNavBar from "~/components/SubNavBar.vue";
import StatTable from "~/components/stats/StatTable.vue";

const ses = getSession();

const route = useRoute();
const gid = computed(() => BigInt(parseInt(String(route.query.gid || "0"))));
const t = computed(() => String(route.query.t || ""));

const cookie = useCookie("EX_COOKIE");
const { data } = await useAsyncData(
  () => `stats-${gid.value}-${t.value}`,
  async () => {
    try {
      const v = await ses.stats(
        gid.value > 1 ? gid.value : undefined,
        t.value ? t.value : undefined,
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
    <NavBar />
    <SubNavBar :nav-items="configNavItems" bold="My Stats" />
    <div v-if="data && !('error' in data)" class="stuffbox">
      <h1 id="gn">{{ data.title }}</h1>
      <p style="margin: 10px auto 0">
        {{
          !(gid && t)
            ? "Your galleries have had a total of"
            : "This gallery has had a total of"
        }}
        <strong>{{ data.total.toLocaleString("en-US") }}</strong> visits.
      </p>
      <table
        v-if="data.ranking"
        style="border-collapse: collapse; margin: 10px auto; text-align: left"
      >
        <tbody>
          <tr>
            <th style="border-bottom: 1px solid #5c0d12" />
            <th
              style="
                width: 70px;
                border-bottom: 1px solid #5c0d12;
                text-align: center;
              "
              colspan="2"
            >
              Ranking
            </th>
            <th
              style="
                width: 60px;
                border-bottom: 1px solid #5c0d12;
                text-align: right;
              "
            >
              Score
            </th>
          </tr>
          <tr>
            <td>
              <NuxtLink
                to="/toplist?tl=11"
                style="text-decoration: none; font-weight: bold"
              >
                All-Time</NuxtLink
              >
            </td>
            <td colspan="3" style="font-style: italic; text-align: right">
              {{ data.ranking?.all }}
            </td>
          </tr>
          <tr>
            <td>
              <NuxtLink
                to="/toplist?tl=12"
                style="text-decoration: none; font-weight: bold"
              >
                Past Year</NuxtLink
              >
            </td>
            <td colspan="3" style="font-style: italic; text-align: right">
              {{ data.ranking?.year }}
            </td>
          </tr>
          <tr>
            <td>
              <NuxtLink
                to="/toplist?tl=13"
                style="text-decoration: none; font-weight: bold"
              >
                Past Month</NuxtLink
              >
            </td>
            <td colspan="3" style="font-style: italic; text-align: right">
              {{ data.ranking?.month }}
            </td>
          </tr>
          <tr>
            <td>
              <NuxtLink
                to="/toplist?tl=15"
                style="text-decoration: none; font-weight: bold"
              >
                Yesterday</NuxtLink
              >
            </td>
            <td colspan="3" style="font-style: italic; text-align: right">
              {{ data.ranking?.yesterday }}
            </td>
          </tr>
        </tbody>
      </table>
      <div id="graphs">
        <div style="flex-shrink: 1; overflow: auto">
          <StatTable :data="data.daily" />
          <p>Daily Stats</p>
        </div>
        <div>
          <StatTable :data="data.monthly" />
          <p>Last 12 Months</p>
        </div>
        <div>
          <StatTable :data="data.yearly" />
          <p>Yearly Stats</p>
        </div>
      </div>
    </div>
  </div>
</template>
<style scoped>
body {
  font-size: 10pt;
}

a {
  text-decoration: none;
}

.stuffbox {
  min-width: 750px;
  max-width: 1000px;
  margin: 10px auto;
  padding: 3px;
}

#graphs {
  display: flex;
  flex-direction: row;
  justify-content: space-around;
  align-items: center;
  flex-wrap: wrap;
}

#graphs > div > table {
  padding: 5px 10px;
  margin: auto;
  border-collapse: collapse;
  background: #f2f0e4;
  border: 1px solid #c2c1c1;
  border-bottom: 0;
}

#graphs > div > p {
  margin: 2px auto 10px;
}

#graphs td {
  min-width: 16px;
}
</style>
