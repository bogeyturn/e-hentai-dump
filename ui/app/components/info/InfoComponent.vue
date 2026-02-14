<script setup lang="ts">
import TopComponent from "~/components/info/top/TopComponent.vue";
import CommentsComponent from "~/components/info/CommentsComponent.vue";
import ImagePagination from "~/components/ImagePagination.vue";
import type { Info } from "exx";
import ReportComponent from "~/components/info/ReportComponent.vue";
import ExplungeComponent from "~/components/info/ExplungeComponent.vue";

const page = defineModel<number>("page");

function patchCssUrl(css: string, ratio: [number, number]) {
  const str = `aspect-ratio: ${ratio[0]} / ${ratio[1]};width: ${ratio[0]}px;background: `;
  const match = css.match(/url\((https?:\/\/[^)]+)\)/);

  if (match && match[1]) {
    return `${str}${css.replace(match[1], wrapUrl(match[1]))}`;
  } else {
    return `${str}${css}`;
  }
}

defineEmits(["close"]);

defineProps<{ galleryData: Info; explunge?: boolean; report?: boolean }>();
</script>

<template>
  <TopComponent :data="galleryData" @close="$emit('close')" />
  <div id="asm" />
  <div v-if="galleryData.newer.length > 0" id="gnd">
    <p style="font-weight: bold">
      There are newer versions of this gallery available:
    </p>
    <NuxtLink
      v-for="(item, idx) in galleryData.newer"
      :key="idx"
      :to="`/g/${item[0]}/${item[1]}/`"
      >{{ item[0] }}
    </NuxtLink>
  </div>
  <template v-if="explunge"><ExplungeComponent /></template>
  <template v-else-if="report"><ReportComponent /></template>
  <template v-else>
    <ImagePagination
      :total-items="galleryData.files"
      :current-page="page"
      :page-size="galleryData.per_page"
      :show-counter="true"
      @update:page="page = $event"
    />
    <div id="gdt" class="gt200">
      <NuxtLink
        v-for="item in galleryData.pages"
        :key="item.key"
        :to="`/s/${item.key}/${galleryData.id}-${item.id}`"
      >
        <div
          title="Page {{ item.id }}"
          :style="patchCssUrl(item.url, item.ratio)"
        />
      </NuxtLink>
    </div>
    <ImagePagination
      :total-items="galleryData.files"
      :page-size="galleryData.per_page"
      :current-page="page"
      :show-counter="false"
      :top="false"
      @update:page="page = $event"
    />
    <CommentsComponent :comments="galleryData.comments" />
  </template>
</template>

<style scoped></style>
