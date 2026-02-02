<script setup lang="ts">
import { configNavItems } from "~/utils/config";
import NavBar from "~/components/NavBar.vue";
import SubNavBar from "~/components/SubNavBar.vue";

const ses = getSession();

const cookie = useCookie("EX_COOKIE");
const { data } = await useAsyncData(
  () => `hathperks`,
  async () => {
    try {
      const v = await ses.perks();
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
    <SubNavBar :nav-items="configNavItems" bold="Hath Perks" />
    <div v-if="data && !('error' in data)" class="stuffbox">
      <h1>Hath Perks</h1>

      <div
        style="
          font-size: 9pt;
          padding: 10px 20px;
          margin: auto;
          text-align: justify;
        "
      >
        <p>
          By running the Hentai@Home client, you will over time gain special
          bonus points known as <span style="font-style: italic">Hath</span>.
          These are rewards for people who help keeping this site free, fast and
          responsive by donating bandwidth and computer resources, and can be
          exchanged here for <span style="font-style: italic">Hath Perks</span>,
          which grant beneficial effects on E-Hentai Galleries and in the
          HentaiVerse.
        </p>
        <p>
          If running H@H is not an option, you can exchange Credits for Hath at
          the
          <NuxtLink style="font-weight: bold" to="/exchange?type=hath"
            >Hath Exchange
          </NuxtLink>
          .
        </p>
        <p>
          While the Hath Perks for the HentaiVerse cannot be obtained in any
          other way, most of the ones that are specific for Galleries will also
          get unlocked by making a donation on the
          <NuxtLink
            style="font-weight: bold"
            to="https://e-hentai.org/bitcoin.php"
            >Donation Screen
          </NuxtLink>
          . These will be refunded if you buy them for Hath, and later make a
          qualifying donation. There is also an option to "adopt" H@H clients
          that will grant you Hath over time as if you were running it yourself.
        </p>
        <p style="margin-top: 5px; text-align: center">
          You currently have
          <span style="font-weight: bold">{{ data.hath }}</span> Hath.
        </p>
      </div>

      <table
        style="
          text-align: left;
          border: 1px solid black;
          border-collapse: collapse;
          max-width: 850px;
          vertical-align: top;
          margin: 5px auto;
          font-size: 9pt;
        "
      >
        <tbody>
          <tr>
            <th
              style="
                width: 160px;
                text-align: left;
                font-weight: bold;
                padding: 3px;
              "
            >
              Hath Perk
            </th>
            <th style="text-align: left; font-weight: bold; padding: 3px">
              Description
            </th>
            <th style="width: 80px">&nbsp;</th>
          </tr>
          <template v-for="(perk_group, i) in data.perks" :key="i">
            <tr
              v-for="(perk, j) in perk_group"
              :key="j"
              :style="{ opacity: perk.disabled ? 0.7 : 1 }"
            >
              <td
                style="
                  padding: 3px 3px 3px 5px;
                  font-weight: bold;
                  vertical-align: top;
                  border-top: 1px solid #cccccc;
                "
                :style="{ paddingLeft: `${5 + (j > 0 ? 10 : 0)}px` }"
              >
                {{ perk.title }}
              </td>
              <td
                style="
                  padding: 3px 5px;
                  vertical-align: top;
                  border-top: 1px solid #cccccc;
                "
              >
                {{ perk.description }}
                <template v-if="perk.free"
                  ><br /><span style="font-size: 8pt; font-weight: bold"
                    >Free with a ${{ perk.free }} donation.</span
                  >
                </template>
              </td>
              <td
                style="
                  text-align: center;
                  vertical-align: top;
                  padding: 3px;
                  border-top: 1px solid #cccccc;
                "
              >
                {{ perk.price }}
                Hath<br /><input
                  type="submit"
                  name="purchase"
                  value="Purchase"
                  :disabled="perk.disabled || perk.price > data.hath"
                />
              </td>
            </tr>
          </template>
        </tbody>
      </table>
    </div>
  </div>
</template>

<style scoped>
.stuffbox {
  min-width: 500px;
  max-width: 980px;
  margin: 10px auto;
  padding: 5px 5px 15px;
  text-align: left;
}

h1 {
  font-size: 10pt;
  font-weight: bold;
  margin: 3px;
  text-align: center;
}
</style>
