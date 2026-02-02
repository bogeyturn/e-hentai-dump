<script setup lang="ts">
import Baredge from "./Baredge.vue";
import { imageIndex } from "~/utils/imageIndex";

const randomImage = useState<string>("random-image", () => {
  return pickRandomImage(imageIndex);
});
</script>

<template>
  <Baredge />
  <div id="botm">
    <img
      :src="randomImage"
      alt="Welcome to E-Hentai.org, the home of Free Hentai, Free Doujinshi, Free Manga, Free CG Sets, and Free H-Anime"
    />
  </div>
  <Baredge />
</template>

<style scoped>
div#botm {
  width: 100%;
  height: 136px;
  background: #e3e0d1 url(/c/graphics/botm_bg.jpg) repeat-x 0px center;
  position: relative;
  z-index: 2;
  margin: auto;
  text-align: center;
}
div#botm img {
  width: 770px;
  height: 136px;
}
</style>
