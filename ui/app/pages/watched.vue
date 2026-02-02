<script setup lang="ts">
import NavBar from "~/components/NavBar.vue";
import SearchArea from "~/components/home/SearchArea.vue";

const showUser = ref(getCookie("includeGalleries") == "1");
const showFav = ref(getCookie("includeFavGalleries") == "1");
</script>
<template>
  <div>
    <NavBar />
    <div class="ido" style="max-width: 1370px">
      <h1 class="ih">Watched Tag Galleries</h1>
      <SearchArea
        v-model:showuser="showUser"
        v-model:showfav="showFav"
        :file-search="false"
      />
    </div>
  </div>
</template>
<style lang="css" src="~/styles/index.css" scoped></style>
