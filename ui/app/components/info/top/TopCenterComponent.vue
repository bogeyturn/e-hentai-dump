<script setup lang="ts">
import { lookup } from "@/utils/tags/collected";

const props = defineProps<{ tags: string[] }>();
function getItemStyle(item: string) {
  if (!lookup[item]) return {};
  return {
    color: "#f1f1f1",
    borderColor: lookup[item][0],
    background: `radial-gradient(${lookup[item][0]}, ${lookup[item][1]})`,
  };
}
const groupedMap = computed(() => {
  const keyOrder = ["language", "group", "artist", "female", "male", "other"];
  const map: Record<string, string[]> = {};
  for (const key of keyOrder) map[key] = [];

  for (const item of props.tags) {
    const [key, value] = item.split(":");
    if (key == undefined || value == undefined) continue;
    if (key in map) map[key]!.push(value);
    else map[key] = [value];
  }

  for (const key of keyOrder) {
    if (map[key] && map[key].length === 0) delete map[key];
  }

  return map;
});
</script>

<template>
  <div id="gd4">
    <div id="taglist">
      <table>
        <tbody>
          <tr v-for="(values, key) in groupedMap" :key="key">
            <td class="tc">{{ key }}:</td>
            <td>
              <div
                v-for="value in values"
                :key="value"
                class="gt"
                :style="getItemStyle(value)"
              >
                <a>{{ value }}</a>
              </div>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
    <div id="tagmenu_act" style="display: none" />
    <div id="tagmenu_new">
      <form action="" method="post" class="nopm" @submit.prevent="() => {}">
        <input
          id="newtagfield"
          type="text"
          name="tags"
          placeholder="Enter new tags, separated with comma"
          size="60"
          maxlength="200"
        /><input id="newtagbutton" type="submit" name="submit" value="Tag" />
      </form>
    </div>
    <div id="gwrd">
      <img
        id="waitroller"
        src="/img/roller.gif"
        style="visibility: hidden"
        alt=""
      />
    </div>
  </div>
</template>

<style scoped></style>
