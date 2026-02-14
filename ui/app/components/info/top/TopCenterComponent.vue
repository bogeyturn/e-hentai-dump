<script setup lang="ts">
import { lookup } from "@/utils/tags/collected";

const props = defineProps<{ tags: string[] }>();
const currentSelected = ref<string | null>(null);

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
                @click="
                  () => {
                    const k = `${key}:${value}`;
                    if (currentSelected == k) {
                      currentSelected = null;
                    } else {
                      currentSelected = k;
                    }
                  }
                "
              >
                <a
                  :style="
                    currentSelected == `${key}:${value}` ? 'color: blue' : ''
                  "
                  >{{ value }}</a
                >
              </div>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
    <div v-if="currentSelected" id="tagmenu_act">
      <img src="/g/mr.gif" class="mr" alt="&gt;" />
      <a>Vote Up</a>
      <img src="/g/mr.gif" class="mr" alt="&gt;" />
      <a>Vote Down</a>
      <img src="/g/mr.gif" class="mr" alt="&gt;" />
      <NuxtLink :to="`/tag/${currentSelected}`">Show Tagged Galleries</NuxtLink>
      <img src="/g/mr.gif" class="mr" alt="&gt;" />
      <NuxtLink
        :to="`https://ehwiki.org/wiki/${currentSelected.split(':', 2)[1]}`"
        target="_blank"
        rel="noopener noreferrer"
        >Show Tag Definition</NuxtLink
      >
      <img src="/g/mr.gif" class="mr" alt="&gt;" />
      <a @click="currentSelected = null">Add New Tag</a>
    </div>
    <div v-else id="tagmenu_new">
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
