<script setup lang="ts">
import NavBar from "~/components/NavBar.vue";
import ManageSubListGroup from "~/components/upload/ManageSubListGroup.vue";

const ses = getSession();

const cookie = useCookie("EX_COOKIE");

const { data } = await useAsyncData(
  () => `upload-manage`,
  async () => {
    try {
      const v = await ses.list_upload();
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
  <div>
    <NavBar />
    <SubNavBar :nav-items="uploadNavItems" bold="Gallery List" />
    <div v-if="data && !('error' in data)" id="m">
      <form>
        <ManageSubListGroup :unpublished="true" :lists="data.unpublished" />
        <ManageSubListGroup :unpublished="false" :lists="data.published" />
        <table id="t">
          <tbody>
            <tr>
              <td>
                Set public category for selected galleries:
                <select name="publiccat">
                  <option value="2" selected>Doujinshi</option>
                  <option value="3">Manga</option>
                  <option value="4">Artist CG</option>
                  <option value="5">Game CG</option>
                  <option value="10">Western</option>
                  <option value="9">Non-H</option>
                  <option value="6">Image Set</option>
                  <option value="7">Cosplay</option>
                  <option value="1">Misc</option>
                </select>
                <input type="submit" name="categorize" value="Go" />
              </td>
            </tr>
            <tr>
              <td>
                Move selected galleries to folder:
                <select name="folderid">
                  <option value="1">1</option>
                  <option value="2">2</option>
                  <option value="0" selected>Unsorted</option>
                </select>
                <input type="submit" name="folderize" value="Go" />
              </td>
            </tr>
          </tbody>
        </table>
      </form>
    </div>
  </div>
</template>
<style scoped>
body {
  font-size: 9pt;
}

a {
  text-decoration: none;
}

h2 {
  font-weight: bold;
  font-size: 10pt;
}

div#m {
  width: 980px;
  margin: -5px auto 10px auto;
  clear: both;
  text-align: left;
}

table#t {
  width: 100%;
}

table#t td {
  text-align: right;
  margin-top: 5px;
  padding-right: 10px;
}

table#t select {
  width: 200px;
}

p#c a {
  font-weight: bold;
}

th {
  padding: 2px;
}

td.l span {
  font-style: italic;
  font-weight: bold;
}

td.l a {
  font-family: courier, serif;
  position: relative;
  top: -1px;
}

td.r span {
  margin: 0 2px;
  vertical-align: middle;
  cursor: pointer;
}
</style>
