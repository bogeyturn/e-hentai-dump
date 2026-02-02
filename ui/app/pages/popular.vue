<script lang="ts" setup>
import NavBar from "~/components/NavBar.vue";
import CommonView from "~/components/views/CommonView.vue";
const ses = getSession();

const cookie = useCookie("EX_COOKIE");

const { data } = await useAsyncData(
  () => `popular`,
  async () => {
    try {
      const v = await ses.popular();
      const cookie_val = await ses.cookie();
      if (cookie_val != cookie.value) cookie.value = cookie_val;
      return v;
    } catch (err) {
      return { error: err };
    }
  },
  { server: true, immediate: true },
);

const view = useCookie("search-view", {
  default: () => "t",
});
</script>
<template>
  <div>
    <NavBar />
    <div class="ido" :style="view == 't' ? 'max-width: 1370px' : ''">
      <div id="toppane">
        <h1 class="ih">Currently Popular Recent Galleries</h1>
      </div>
      <div style="position: relative; z-index: 2">
        <CommonView
          :first="true"
          :last="true"
          :query="''"
          :prev="null"
          :next="null"
          :items="data"
        />
      </div>
    </div>
  </div>
</template>
