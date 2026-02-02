<script setup lang="ts">
import type { SearchInfo } from "~~/pkg";
import Category from "~/components/views/Category.vue";
import Stars from "~/components/views/Stars.vue";

defineProps({
  items: {
    type: Array<SearchInfo>,
    required: true,
  },
});

function displayTag(tag: string) {
  const splits = tag.split(":", 2);
  if (splits[0] == "female") {
    return "f:" + splits[1];
  } else if (splits[0] == "male") {
    return "m:" + splits[1];
  } else if (splits[0] == "other") {
    return "x:" + splits[1];
  }
  return splits[1];
}

const hoveredRow = ref(null);
</script>

<template>
  <table class="itg gltc">
    <tbody>
      <tr>
        <th />
        <th>Published</th>
        <th>Title</th>
        <th class="glhide">Uploader</th>
      </tr>
      <tr :data-new="item.new" v-for="(item, i) in items" :key="i">
        <td class="gl1c glcat">
          <Category :category="item.category" class="cn" />
        </td>
        <td class="gl2c">
          <div
            class="glcut"
            :style="`visibility: ${hoveredRow == i ? 'visible' : 'hidden'};`"
          />
          <div
            class="glthumb"
            :style="`${i < items.length - 3 ? (i <= 3 ? 'top: 0;' : 'top: 0; transform: translateY(-50%);') : 'bottom: 0;'} visibility: ${hoveredRow == i ? 'visible' : 'hidden'};`"
          >
            <div>
              <img
                style="width: 250px"
                :alt="item.title"
                :title="item.title"
                :src="wrapUrl(item.img ?? '')"
              />
            </div>
            <div>
              <div>
                <Category :category="item.category" class="cn" />
                <div :class="item.new ? 'glnew' : ''">
                  {{ item.published }}
                </div>
              </div>
              <div>
                <Stars :star="item.rating" :opacity="1" color="" />
                <div>{{ item.pages }} pages</div>
              </div>
            </div>
          </div>
          <div>
            <div :class="item.new ? 'glnew' : ''">
              {{ item.published }}
            </div>
            <Stars :star="item.rating" :opacity="1" color="" />
            <div class="gldown">
              <NuxtLink
                v-if="false"
                :to="`/gallerytorrents?gid=${item.id}&amp;t=${item.token}`"
                rel="nofollow"
                ><img src="/g/t.png" alt="T" title="Show torrents"
              /></NuxtLink>
              <img src="/g/td.png" alt="T" title="No torrents available" />
            </div>
          </div>
        </td>
        <td
          class="gl3c glname"
          @mouseover="hoveredRow = i"
          @mouseout="hoveredRow = null"
        >
          <NuxtLink :href="`/g/${item.id}/${item.token}/`">
            <div class="glink">{{ item.title }}</div>
            <div>
              <div
                v-for="(tag, j) in item.tags"
                class="gt"
                :title="tag"
                :key="j"
              >
                {{ displayTag(tag) }}
              </div>
            </div>
          </NuxtLink>
        </td>
        <td class="gl4c glhide">
          <div>
            <NuxtLink :to="`/uploader/${item.publisher}`">{{
              item.publisher
            }}</NuxtLink>
          </div>
          <div>{{ item.pages }} pages</div>
        </td>
      </tr>
    </tbody>
  </table>
</template>
