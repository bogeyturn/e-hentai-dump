<script setup lang="ts">
import NavBar from "~/components/NavBar.vue";
import Banner from "~/components/news/Banner.vue";
import Baredge from "~/components/news/Baredge.vue";

const ses = getSession();

const cookie = useCookie("EX_COOKIE");

const { data } = await useAsyncData(
  () => `news`,
  async () => {
    try {
      const v = await ses.news();
      const cookie_val = await ses.cookie();
      if (cookie_val != cookie.value) cookie.value = cookie_val;
      return v;
    } catch (err) {
      return { error: err };
    }
  },
  { server: true, immediate: true },
);
</script>

<template>
  <!-- eslint-disable vue/no-v-html -->

  <div id="newsouter">
    <Banner />
    <div
      v-if="data && Array.isArray(data) && data[0] && data[1]"
      id="newsinner"
    >
      <NavBar />
      <table id="nt">
        <tbody>
          <tr>
            <td class="nd">
              <div class="nwo">
                <h2>
                  <a :href="data[0].link">{{ data[0].title }}</a>
                </h2>
                <div class="nwi" v-html="data[0].content" />
              </div>
              <div class="nwo">
                <h2>
                  <a :href="data[1].link">{{ data[1].title }}</a>
                </h2>

                <div class="nwi" v-html="data[1].content" />
                <div class="nwu">
                  Changelogs: &nbsp;
                  <a
                    href="https://forums.e-hentai.org/index.php?showtopic=244935"
                    >2020+</a
                  >&nbsp;
                  <a
                    href="https://forums.e-hentai.org/index.php?showtopic=225132"
                    >2019</a
                  >&nbsp;
                  <a
                    href="https://forums.e-hentai.org/index.php?showtopic=212243"
                    >2018</a
                  >&nbsp;
                  <a
                    href="https://forums.e-hentai.org/index.php?showtopic=200886"
                    >2017</a
                  >&nbsp;
                  <a
                    href="https://forums.e-hentai.org/index.php?showtopic=188685"
                    >2016</a
                  >&nbsp;
                  <a
                    href="https://forums.e-hentai.org/index.php?showtopic=181572"
                    >2015</a
                  >
                </div>
              </div>
            </td>
            <td class="nd">
              <div class="nwo">
                <div
                  v-for="item in data.slice(2)"
                  :key="item.link"
                  class="newstable"
                >
                  <div class="newstitle">
                    <a :href="item.link">{{ item.title }}</a>
                  </div>
                  <div class="newsdate">
                    {{ item.date }}
                  </div>
                  <div class="newstext" v-html="item.content" />

                  <div class="newslink" v-html="item.link2" />
                </div>

                <div style="text-align: center">
                  [<a href="https://forums.e-hentai.org/index.php?showforum=2"
                    >Show Older News</a
                  >]
                </div>
              </div>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
    <div v-if="data && !Array.isArray(data)">{{ data.error }}</div>
    <Baredge />
  </div>
</template>

<style scoped>
h1 {
  font-size: 10pt;
  font-weight: bold;
  margin: 3px;
  text-align: center;
}
h2 {
  font-weight: bold;
  font-size: 12pt;
  text-align: left;
  border-bottom: 1px solid #5c0d11;
}
h2 a {
  text-decoration: none;
}
div {
  font-size: 9pt;
}
div.nwo {
  width: 95%;
  padding: 2px;
  margin: auto;
}
div.nwi {
  margin: 1px auto 4px 10px;
}
div.nwf {
  margin: 10px auto 4px 10px;
}
div.nwu {
  margin: 20px auto 4px 10px;
  font-size: 12pt;
}
div.nwu a {
  font-weight: bold;
  text-decoration: none;
}
td.twt {
  width: 79px;
  padding-right: 3px;
  text-align: right;
  font-weight: bold;
  font-style: italic;
}
div.newstable {
  margin: 1px auto 5px;
}
div.newstitle {
  border: 0;
  border-bottom: 1px solid #5c0d11;
  text-align: left;
  padding: 7px 0 5px 5px;
}
div.newstitle a {
  font-weight: bold;
  font-size: 12pt;
  text-decoration: none;
}
div.newsdate {
  border: 0;
  text-align: left;
  padding: 4px 0 2px 7px;
}
div.newslink {
  border: 0;
  text-align: left;
  padding: 5px 20px;
}
div.newslink a {
  font-weight: bold;
  font-style: italic;
  text-decoration: none;
}
div.newstext {
  border: 0;
  text-align: justify;
  padding: 5px 20px;
  font-size: 9pt;
}
div.newstext a {
  text-decoration: underline;
  font-size: 9pt;
}
div#newsouter {
  width: 100%;
  overflow: hidden;
  margin: auto;
  border: 1px solid #5c0d12;
}
div#newsinner {
  font-size: 10pt;
  border: 0;
  text-align: center;
  vertical-align: top;
}

table#nt {
  width: 99%;
  margin-top: 3px;
  text-align: left;
}
td.nd {
  width: 50%;
  vertical-align: top;
}

div.bartop {
  width: 100%;
  height: 5px;
  background-image: url(/c/graphics/bar.png);
  position: relative;
  z-index: 2;
}
</style>
