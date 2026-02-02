<script setup lang="ts">
import TitleBar from "~/components/info/top/TitleBar.vue";
import ImageSideBar from "~/components/info/top/ImageSideBar.vue";
import TopLeftComponent from "~/components/info/top/TopLeftComponent.vue";
import TopCenterComponent from "~/components/info/top/TopCenterComponent.vue";
import TopRightComponent from "~/components/info/top/TopRightComponent.vue";
import type {Info} from "exx";
import ActualTopRightComponent from "~/components/info/top/ActualTopRightComponent.vue";

defineProps<{ data: Info }>();

defineEmits(["close"]);

const tagField = ref("");
</script>

<template>
  <div class="gm">
    <ImageSideBar :width="250" :height="340" url=""/>
    <TitleBar :title="data.title" :alt-title="data.alt_title"/>
    <div id="gmid">
      <TopLeftComponent
          :idd="BigInt(data.id)"
          :apiuid="BigInt(data.apiuid)"
          :uploader="data.uploader"
          :visible="data.visible"
          :rating="data.rating"
          :size="data.size"
          :posted="data.posted"
          :title="data.title"
          :apikey="data.apikey"
          :category="data.category"
          :files="data.files"
          :language="data.language"
          :my-stars="data.my_stars"
          :token="data.token"
          @close="$emit('close')"
      />
      <TopCenterComponent :tags="data.tags"/>

      <!-- <ActualTopRightComponent
        :gid="BigInt(data.id)"
        :token="data.token"
      />-->
      <TopRightComponent
          v-model:tag-field="tagField" :idd="BigInt(data.id)" :token="data.token"
          :favorite="data.favorite" @close="$emit('close')"/>
      <div class="c"/>
    </div>
    <div class="c"/>
  </div>
</template>

<style scoped></style>
