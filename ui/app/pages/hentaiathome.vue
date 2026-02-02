<script setup lang="ts">
import { configNavItems } from "~/utils/config";
import NavBar from "~/components/NavBar.vue";
import SubNavBar from "~/components/SubNavBar.vue";
const ses = getSession();

const cookie = useCookie("EX_COOKIE");
const { data } = await useAsyncData(
  () => `h@h`,
  async () => {
    try {
      const v = await ses.hentai_at_home();
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
    <SubNavBar :nav-items="configNavItems" bold="Hentai@Home" />
    <div v-if="data && !('error' in data)" class="stuffbox">
      <h1>Hentai@Home Clients</h1>
      <div>
        <table id="hathstats">
          <tbody>
            <tr>
              <th style="text-align: right; padding-right: 10px">H@H Region</th>
              <th colspan="3">Current Network Load</th>
              <th>Hits/sec</th>
              <th>Hits/GB</th>
              <th style="width: 65px">Quality</th>
            </tr>
            <tr v-for="(item, i) in data.clients" :key="i">
              <td>{{ item.name }}</td>
              <td style="padding: 1px 5px 1px 20px">
                {{ ((item.load_bytes * 8) / 1_000_000_000).toFixed(2) }} Gbit/s
              </td>
              <td style="padding: 1px 0">=</td>
              <td style="padding: 1px 20px 1px 5px; text-align: right">
                {{ item.load_bytes / 1_000_000 }} MB/s
              </td>
              <td>{{ item.hits_sec ?? "-" }}</td>
              <td>{{ item.hits_gb ?? "-" }}</td>
              <td style="text-align: center">{{ item.quality ?? "-" }}</td>
            </tr>
          </tbody>
        </table>

        <div
          style="
            text-align: center;
            font-size: 8pt;
            font-weight: bold;
            padding-top: 5px;
          "
        >
          Hits/GB shows the current average number of hits per minute per
          gigabyte of allocated disk space for all online clients in the
          region.<br />
          A high number indicates that clients are more needed in the region
          than other regions. A low number indicates oversaturation.
        </div>
      </div>
      <div style="margin: auto; max-width: 700px">
        <h2>Apply for H@H participation</h2>
        <p>
          Due to excess capacity, we are not taking new applications for Europe
          at the moment, with an exception for donators.
        </p>
        <p>
          If you want to run a client <strong>outside of Europe</strong>,
          <a
            href="https://forums.e-hentai.org/index.php?act=Msg&amp;CODE=4&amp;MID=6"
            >PM Tenboro</a
          >. Include the system specs, speedtest and location in the message.
        </p>
        <p style="font-weight: bold">
          The client will be locked to the provided location. Do not send PMs
          asking to run clients in Europe.
        </p>
      </div>
      <h2>Client Download</h2>

      <div style="max-width: 900px; margin: auto">
        <table style="margin: 5px auto; border-collapse: collapse">
          <tbody>
            <tr>
              <th style="width: 125px">File</th>
              <th style="width: 90px">Size</th>
              <th style="width: 460px">SHA-256</th>
            </tr>
            <tr
              v-for="(item, i) in data.client_download"
              :key="i"
              style="
                border-top: 1px solid #d9d7cc;
                border-bottom: 1px solid #d9d7cc;
                height: 22px;
              "
            >
              <td>
                <NuxtLink style="font-weight: bold" :to="item.link">{{
                  item.file
                }}</NuxtLink>
              </td>
              <td>{{ item.size }} bytes</td>
              <td style="font-family: courier, serif">{{ item.hash }}</td>
            </tr>
          </tbody>
        </table>

        <p style="text-align: center">
          You can find the current release notes
          <a href="https://forums.e-hentai.org/index.php?showtopic=234458"
            >here</a
          >. You should verify that the size and cryptographic hash correspond
          to the files you download. Hentai@Home is an Open Source project
          released under the GNU General Public Licence v3. The source code and
          build scripts for Windows and Linux-like systems can be found above.
        </p>
      </div>
    </div>
  </div>
</template>

<style scoped>
.stuffbox {
  min-width: 740px;
  max-width: 1060px;
  margin: 10px auto;
  padding: 5px 5px 15px;
  text-align: left;
  font-size: 9pt;
}

.infot {
  text-align: justify;
  border-collapse: collapse;
}

.infota {
  width: 370px;
  vertical-align: top;
  border-bottom: 1px solid #5c0d12;
  border-right: 1px dashed #5c0d12;
  padding: 5px;
}

.infota > div:first-child {
  font-weight: bold;
  padding-bottom: 2px;
}

.infota > div:last-child {
  padding: 1px 3px 1px 8px;
}

.infotv {
  width: 700px;
  vertical-align: top;
  border-bottom: 1px solid #5c0d12;
  padding: 5px 10px;
}

h1 {
  font-size: 10pt;
  font-weight: bold;
  margin: 3px;
  text-align: center;
}

h2 {
  font-size: 9pt;
  font-weight: bold;
  margin: 15px 0 3px;
  text-align: center;
}

#hathstats {
  margin: 20px auto 0;
  font-size: 12pt;
  border-collapse: collapse;
}

#hathstats th {
  padding: 3px;
  text-align: center;
}

#hathstats td {
  padding: 3px 15px;
  text-align: right;
}

#hathstats tr:last-child td {
  border-top: 1px solid #5c0d1269;
}

#hct {
  max-width: 1050px;
  margin: 5px auto;
}

#hct td {
  text-align: right;
  padding: 2px 3px;
}

#hct th {
  text-align: center;
  padding: 2px 3px;
}

@media screen and (max-width: 980px) {
  #hct td:nth-child(2),
  #hct th:nth-child(2),
  #hct td:nth-child(4),
  #hct th:nth-child(4) {
    display: none;
  }
}

@media screen and (max-width: 880px) {
  #hct td:nth-child(8),
  #hct th:nth-child(8),
  #hct td:nth-child(9),
  #hct th:nth-child(9) {
    display: none;
  }
}
</style>
