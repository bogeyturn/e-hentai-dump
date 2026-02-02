<script setup lang="ts">
import { configNavItems } from "~/utils/config";
import NavBar from "~/components/NavBar.vue";
import SubNavBar from "~/components/SubNavBar.vue";
import { computed, useRoute } from "#imports";
import type { CreditLog, KarmaPage } from "exx";

const route = useRoute();
const router = useRouter();

const ses = getSession();

const credits = computed(
  () => String(route.query.t || "").toLowerCase() == "credits",
);

const unit = computed(() =>
  String(route.query.t || "").toLowerCase() == "credits" ? "Credit" : "Karma",
);

const cookie = useCookie("EX_COOKIE");
const { data } = await useAsyncData(
  () => `logs-${credits.value}`,
  async () => {
    try {
      let v;
      if (credits.value) {
        v = await ses.credit_logs();
      } else {
        v = await ses.karma_logs();
      }
      const cookie_val = await ses.cookie();
      if (cookie_val != cookie.value) cookie.value = cookie_val;
      return v;
    } catch (err) {
      return { error: err };
    }
  },
  { server: true, immediate: true },
);

const isCreditLogArray = (v: unknown): v is CreditLog[] => {
  return Array.isArray(v) && v.length > 0 && "information" in v[0];
};

const isKarmaLogArray = (v: unknown): v is KarmaPage => {
  return typeof v === "object" && v !== null && "karma" in v;
};

watchEffect(() => {
  const t = String(route.query.t || "");
  if (t !== "credits" && t !== "karma") {
    router.replace({
      query: {
        ...route.query,
        t: "karma",
      },
    });
  }
});
</script>

<template>
  <div>
    <NavBar />
    <SubNavBar :nav-items="configNavItems" :bold="`${unit} Log`" />
    <div v-if="data && isCreditLogArray(data)" class="logbox">
      <table>
        <tbody>
          <tr>
            <th
              style="
                text-align: left;
                border-bottom: 1px solid #5c0d12;
                width: 100px;
              "
            >
              Date
            </th>
            <th
              style="
                text-align: right;
                border-bottom: 1px solid #5c0d12;
                width: 50px;
                padding-right: 10px;
              "
            >
              Amount
            </th>
            <th style="text-align: left; border-bottom: 1px solid #5c0d12">
              Information
            </th>
          </tr>

          <tr v-for="(item, i) in data" :key="i">
            <td style="text-align: left; vertical-align: top">
              {{ displayDate(item.date) }}
            </td>
            <td
              style="
                text-align: right;
                vertical-align: top;
                padding-right: 10px;
              "
            >
              <span :style="{ color: item.ammount < 0 ? 'red' : 'green' }"
                >{{ item.ammount >= 0 ? "+" : ""
                }}{{ item.ammount.toLocaleString("en-US") }}</span
              >
            </td>
            <td style="text-align: left; vertical-align: top">
              <div style="overflow: hidden">{{ item.information }}</div>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
    <div
      v-if="data && isKarmaLogArray(data)"
      style="font-weight: bold; margin-top: 5px; font-size: 10pt"
    >
      Total Karma:
      <span :style="{ color: data.karma < 0 ? 'red' : 'green' }">{{
        data.karma
      }}</span>
    </div>
    <div v-if="data && isKarmaLogArray(data)" class="logbox">
      <table>
        <tbody>
          <tr>
            <th
              style="
                text-align: left;
                border-bottom: 1px solid #5c0d12;
                width: 100px;
              "
            >
              Date
            </th>
            <th
              style="
                text-align: right;
                border-bottom: 1px solid #5c0d12;
                width: 50px;
                padding-right: 10px;
              "
            >
              Karma
            </th>
            <th
              style="
                text-align: left;
                border-bottom: 1px solid #5c0d12;
                width: 100px;
              "
            >
              From
            </th>
            <th
              style="
                text-align: left;
                border-bottom: 1px solid #5c0d12;
                width: 60px;
              "
            >
              Topic
            </th>
            <th style="text-align: left; border-bottom: 1px solid #5c0d12">
              Comment
            </th>
          </tr>

          <tr v-for="(item, i) in data.logs" :key="i">
            <td style="text-align: left; vertical-align: top">
              {{ displayDate(item.date) }}
            </td>
            <td
              style="
                text-align: right;
                vertical-align: top;
                padding-right: 10px;
                color: green;
              "
            >
              <span :style="{ color: item.ammount < 0 ? 'red' : 'green' }"
                >{{ item.ammount >= 0 ? "+" : ""
                }}{{ item.ammount.toLocaleString("en-US") }}</span
              >
            </td>
            <td style="text-align: left; vertical-align: top">
              <NuxtLink
                :to="`https://forums.e-hentai.org/index.php?showuser=${item.from_id}`"
                target="_nk"
                >{{ item.from }}
              </NuxtLink>
            </td>
            <td style="text-align: left; vertical-align: top">
              {{ item.topic }}
            </td>
            <td style="text-align: left; vertical-align: top">
              {{ item.comment }}
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<style scoped>
.logbox {
  text-align: center;
  max-width: 800px;
  min-width: 500px;
  padding: 5px 20px 10px;
  margin: 10px auto 5px;
  background: #edebdf;
  border: 1px solid #5c0d12;
  font-size: 9pt;
}

table {
  border-collapse: collapse;
  margin: auto;
  width: 95%;
}
</style>
