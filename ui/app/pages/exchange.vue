<script setup lang="ts">
import NavBar from "~/components/NavBar.vue";
import SubNavBar from "~/components/SubNavBar.vue";
import { configNavItems } from "~/utils/config";
import { computed, useRoute } from "#imports";

const route = useRoute();
const router = useRouter();

function get_t() {
  return String(route.query.t || "");
}

const ses = getSession();

const t = computed(() => get_t());

const unit = computed(() => (get_t().toLowerCase() == "hath" ? "Hath" : "GP"));
const unit2 = computed(() =>
  get_t().toLowerCase() == "hath" ? "Hath" : "kGP",
);

async function sell() {
  await ses.exchange_ask(t.value == "hath", ask_count.value, ask_price.value);
  data.value = await reload();
}

async function buy() {
  await ses.exchange_bid(t.value == "hath", bid_count.value, bid_price.value);
  data.value = await reload();
}
const cookie = useCookie("EX_COOKIE");

async function reload() {
  const v = await ses.exchange_info(t.value == "hath");
  const cookie_val = await ses.cookie();
  if (cookie_val != cookie.value) cookie.value = cookie_val;
  return v;
}

const { data } = await useAsyncData(
  () => `exchange-${t.value}`,
  async () => {
    try {
      return await reload();
    } catch (err) {
      return { error: err };
    }
  },
  { server: true, immediate: true },
);

const bid_price = ref(data.value.bid_price);
const bid_count = ref(data.value.bid_count);
const ask_price = ref(data.value.ask_price);
const ask_count = ref(data.value.ask_count);
watch(
  () => data.value?.bid_price,
  (newVal) => {
    bid_price.value = newVal ?? "";
  },
  { immediate: true },
);

watch(
  () => data.value?.bid_count,
  (newVal) => {
    bid_count.value = newVal ?? "";
  },
  { immediate: true },
);
watch(
  () => data.value?.ask_count,
  (newVal) => {
    ask_count.value = newVal ?? "";
  },
  { immediate: true },
);
watch(
  () => data.value?.ask_price,
  (newVal) => {
    ask_price.value = newVal ?? "";
  },
  { immediate: true },
);
watchEffect(() => {
  if (t.value !== "gp" && t.value !== "hath") {
    router.replace({
      query: {
        ...route.query,
        t: "hath",
      },
    });
  }
});
</script>
<template>
  <div>
    <NavBar />
    <SubNavBar :nav-items="configNavItems" :bold="`${unit} Exchange`" />
    <div v-if="data && !('error' in data)" class="stuffbox">
      <h1>The {{ unit }} Exchange</h1>
      <div class="outer">
        <div>
          <h2>Last 8 Hours (per {{ unit2 }})</h2>
          <div>
            <strong>High</strong>:
            {{ data.l8h.high.toLocaleString("en-US") }} Credits &nbsp;
            <strong>Low</strong>:
            {{ data.l8h.low.toLocaleString("en-US") }} Credits &nbsp;
            <strong>Avg</strong>:
            {{ data.l8h.avg.toLocaleString("en-US") }} Credits &nbsp;
            <strong>Vol</strong>: {{ data.l8h.vol.toLocaleString("en-US") }}
            {{ unit2 }}
          </div>
        </div>

        <div>
          <h2>Last 24 Hours (per {{ unit2 }})</h2>
          <div>
            <strong>High</strong>:
            {{ data.l24h.high.toLocaleString("en-US") }} Credits &nbsp;
            <strong>Low</strong>:
            {{ data.l24h.low.toLocaleString("en-US") }} Credits &nbsp;
            <strong>Avg</strong>:
            {{ data.l24h.avg.toLocaleString("en-US") }} Credits &nbsp;
            <strong>Vol</strong>: {{ data.l24h.vol.toLocaleString("en-US") }}
            {{ unit2 }}
          </div>
        </div>
      </div>

      <div class="outer">
        <div>
          <h2>Buy {{ unit }}</h2>
          <div>
            <form id="buyform" action="" method="post">
              Count:
              <input
                id="bid_count"
                v-model="bid_count"
                type="text"
                name="bid_count"
                size="8"
                maxlength="8"
                autocomplete="off"
              />
              {{ unit2 }} &nbsp; Bid Price/{{ unit2 }}:
              <input
                id="bid_price"
                v-model="bid_price"
                type="text"
                name="bid_price"
                size="8"
                maxlength="8"
                autocomplete="off"
              />
              C &nbsp;
              <a
                href="#"
                style="font-weight: bold; text-decoration: none"
                @click="buy"
                >Buy {{ unit }}!</a
              >
            </form>
          </div>
          <div style="margin-top: 5px; font-weight: bold">
            Available: {{ data.credits.toLocaleString("en-US") }} Credits
            <template v-if="bid_price"
              >({{ Math.floor(data.credits / bid_price) }}
              {{ unit2 }})</template
            >
          </div>
        </div>

        <div>
          <h2>Sell {{ unit }}</h2>
          <div>
            <form id="sellform" action="" method="post">
              <input
                type="hidden"
                name="actionkey"
                value="e854cc1721877e8ac681a714d4c02d172924a5c2"
              />
              Count:
              <input
                id="ask_count"
                v-model="ask_count"
                type="text"
                name="ask_count"
                size="8"
                maxlength="8"
                autocomplete="off"
              />
              {{ unit2 }} &nbsp; Ask Price/{{ unit2 }}:
              <input
                id="ask_price"
                v-model="ask_price"
                type="text"
                name="ask_price"
                size="8"
                maxlength="8"
                autocomplete="off"
              />
              C &nbsp;
              <a
                href="#"
                style="font-weight: bold; text-decoration: none"
                @click="sell"
                >Sell {{ unit }}!</a
              >
            </form>
          </div>
          <div style="margin-top: 5px; font-weight: bold">
            Available: {{ data.currency.toLocaleString("en-US") }} {{ unit2 }}
            <template v-if="ask_price"
              >({{
                (data.currency * ask_price).toLocaleString("en-US")
              }}
              Credits)</template
            >
          </div>
        </div>
      </div>

      <div class="outer">
        <div>
          <h2>Active Orders</h2>

          <div style="float: left; width: 220px">
            <h3>Bid (Buyers)</h3>
            <table>
              <tbody>
                <tr v-for="(item, i) in data.active_bid" :key="i">
                  <td style="text-align: right">
                    {{ item.vol.toLocaleString("en-US") }} {{ unit2 }}
                  </td>
                  <td style="text-align: center">@</td>
                  <td style="text-align: right">
                    {{ item.price.toLocaleString("en-US") }} Credits
                  </td>
                </tr>
              </tbody>
            </table>
          </div>

          <div style="float: left; width: 220px">
            <h3>Ask (Sellers)</h3>
            <table>
              <tbody>
                <tr v-for="(item, i) in data.active_ask" :key="i">
                  <td style="text-align: right">
                    {{ item.vol.toLocaleString("en-US") }} {{ unit2 }}
                  </td>
                  <td style="text-align: center">@</td>
                  <td style="text-align: right">
                    {{ item.price.toLocaleString("en-US") }} Credits
                  </td>
                </tr>
              </tbody>
            </table>
          </div>

          <div class="c" />
        </div>

        <div>
          <h2>Recent Transactions</h2>
          <table id="historytable">
            <tbody>
              <tr>
                <th>Time</th>
                <th>Seller</th>
                <th>Buyer</th>
                <th style="text-align: right">Volume</th>
                <th style="text-align: center; padding-left: 15px">
                  Unit Cost
                </th>
              </tr>
              <tr v-for="(item, i) in data.recent" :key="i">
                <td>{{ displayTime(item.time) }}</td>
                <td>
                  <div
                    style="
                      position: relative;
                      width: 85px;
                      height: 15px;
                      overflow: hidden;
                    "
                  >
                    {{ item.seller }}
                  </div>
                </td>
                <td>
                  <div
                    style="
                      position: relative;
                      width: 85px;
                      height: 15px;
                      overflow: hidden;
                    "
                  >
                    {{ item.buyer }}
                  </div>
                </td>
                <td style="text-align: right">
                  {{ item.vol.toLocaleString("en-US") }} {{ unit2 }}
                </td>
                <td style="text-align: right">
                  {{ item.price.toLocaleString("en-US") }} Credits
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>
  </div>
</template>
<style scoped>
h2 {
  margin: 15px auto 3px;
  padding: 0 0 2px;
  border-bottom: 1px solid #5c0d12;
  width: 410px;
  font-size: 10pt;
}

h3 {
  margin: 5px auto 3px;
  padding: 0 0 2px;
  font-size: 9pt;
}

#sellform input {
  text-align: right;
}

#buyform input {
  text-align: right;
}

table {
  margin: auto;
}

#historytable {
  text-align: left;
  width: 380px;
}

#historytable th {
  text-align: left;
}

.stuffbox {
  max-width: 980px;
  min-width: 500px;
  margin: 10px auto;
  padding: 3px 3px 15px;
  font-size: 9pt;
}

h1 {
  font-size: 10pt;
  font-weight: bold;
  margin: 3px;
  text-align: center;
}

.outer {
  max-width: 900px;
  margin: 15px auto 0;
  display: flex;
  flex-flow: row wrap;
  justify-content: center;
}

.outer > div {
  width: 449px;
}
</style>
