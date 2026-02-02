<script setup lang="ts">
import type { VisitStat } from "exx";
defineProps<{ data: VisitStat[] }>();
function formatNumber(num) {
  const units = [
    { divisor: 1_000_000_000_000, symbol: "T", zeros: 11 }, // last 11 digits 0
    { divisor: 1_000_000_000, symbol: "B", zeros: 8 }, // last 8 digits 0
    { divisor: 1_000_000, symbol: "M", zeros: 5 }, // last 5 digits 0
    { divisor: 1_000, symbol: "K", zeros: 2 }, // last 2 digits 0
  ];

  for (let { divisor, symbol, zeros } of units) {
    if (num >= divisor && num % 10 ** zeros === 0) {
      let shortNum = num / divisor;
      let rounded = Math.round(shortNum * 10) / 10;
      return rounded % 1 === 0
        ? `${rounded.toFixed(0)}${symbol}`
        : `${rounded}${symbol}`;
    }
  }

  return num.toString(); // no trailing zeros, display full number
}

function calc_style(norm: number) {
  let px = Math.round(200 * norm);
  if (px === 0) px = 1;
  return `height:${px}px; width:8px`;
}
</script>

<template>
  <table>
    <tbody>
      <tr>
        <td :colspan="data.length + 1" style="height: 6px" />
      </tr>
      <tr>
        <td style="height: 202px" />
        <td v-for="(d, i) in data" :key="i" class="stdb">
          <img src="/g/v.png" :style="calc_style(d.visits_norm)" alt="" /><img
            src="/g/h.png"
            :style="calc_style(d.hits_norm)"
          />
        </td>
      </tr>
      <tr>
        <td></td>
        <td class="stdk" v-for="(d, i) in data" :key="i">{{ d.title }}</td>
      </tr>
      <tr>
        <td class="stdk" style="text-align: right">Visits</td>
        <td class="stdv" v-for="(d, i) in data" :key="i">
          {{ formatNumber(d.visits) }}
        </td>
      </tr>
      <tr>
        <td class="stdk" style="text-align: right">Hits</td>
        <td class="stdv" v-for="(d, i) in data">{{ formatNumber(d.hits) }}</td>
      </tr>
    </tbody>
  </table>
</template>

<style scoped>
body {
  font-size: 10pt;
}

a {
  text-decoration: none;
}

.stdb {
  text-align: center;
  vertical-align: bottom;
  margin: 0px auto 0px auto;
  padding: 1px 7px 0px 7px;
}

.stdk {
  font-size: 8pt;
  text-align: center;
  margin: 0px auto 0px auto;
  padding: 1px 3px;
  border: 1px solid #5c0d12;
  font-weight: bold;
}

.stdv {
  font-size: 8pt;
  text-align: center;
  margin: 0px auto 0px auto;
  padding: 1px 3px;
  border: 1px solid #5c0d12;
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
