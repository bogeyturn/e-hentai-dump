<script setup lang="ts">
import TitleBar from "~/components/info/top/TitleBar.vue";
import ImageSideBar from "~/components/info/top/ImageSideBar.vue";
import TopLeftComponent from "~/components/info/top/TopLeftComponent.vue";
import TopCenterComponent from "~/components/info/top/TopCenterComponent.vue";
import type { Info } from "exx";
import ActualTopRightComponent from "~/components/info/top/ActualTopRightComponent.vue";

defineProps<{ data: Info }>();

defineEmits(["close"]);

</script>

<template>
  <div class="gm">
    <ImageSideBar :width="data.thumb.ratio[0]" :height="data.thumb.ratio[1]" :url="data.thumb.url" />
    <TitleBar :title="data.title" :alt-title="data.alt_title" />
    <div id="gmid">

      <TopLeftComponent
        :idd="BigInt(data.id)"
        :apiuid="data.apiuid ? BigInt(data.apiuid) : null"
        :uploader="data.uploader"
        :visible="data.visible"
        :rating="data.rating"
        :size="data.size"
        :posted="data.posted"
        :uploader-id="data.uploader_id"
        :parent="data.parent"
        :favorited="data.favorited"
        :title="data.title"
        :apikey="data.apikey"
        :category="data.category"
        :files="data.files"
        :language="data.language"
        :my-stars="data.my_stars"
        :token="data.token"
        :favorite="data.favorite"
        @close="$emit('close')"
      />
      <TopCenterComponent :tags="data.tags" />

      <ActualTopRightComponent :gid="BigInt(data.id)" :token="data.token" />
      <!-- <TopRightComponent
        v-model:tag-field="tagField"
        :idd="BigInt(data.id)"
        :token="data.token"
        :favorite="data.favorite"
        @close="$emit('close')"
      /> -->
      <div class="c" />
    </div>
    <div class="c" />
  </div>
</template>

<style scoped></style>
