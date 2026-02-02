<script setup lang="ts">
import NavBar from "~/components/NavBar.vue";

const ses = getSession();

const cookie = useCookie("EX_COOKIE");

const { data } = await useAsyncData(
  () => `upload-managefolders`,
  async () => {
    try {
      const v = await ses.folder_info();
      const cookie_val = await ses.cookie();
      if (cookie_val != cookie.value) cookie.value = cookie_val;
      return v;
    } catch (err) {
      return { error: err };
    }
  },
  { server: true, immediate: true },
);
const newFolderName = ref("");

async function createFolder() {
  try {
    if (newFolderName.value.length < 1) {
      return;
    }
    await ses.create_folder(newFolderName.value);
    newFolderName.value = "";
    data.value = await ses.folder_info();
    const cookie_val = await ses.cookie();
    if (cookie_val != cookie.value) cookie.value = cookie_val;
  } catch (err) {
    data.value = { error: err };
  }
}

async function reorderFolders() {
  if (!data.value || "error" in data.value) return;
  try {
    await ses.reorder_folder(data.value);
    data.value = await ses.folder_info();
    const cookie_val = await ses.cookie();
    if (cookie_val != cookie.value) cookie.value = cookie_val;
  } catch (err) {
    data.value = { error: err };
  }
}

async function autoReorder() {
  const confirmed = window.confirm(
    "Are you sure you wish to sort your folders automatically? Doing this will undo the current custom sort order.",
  );
  if (!confirmed) return;
  try {
    await ses.auto_reorder_folder();
    data.value = await ses.folder_info();
    const cookie_val = await ses.cookie();
    if (cookie_val != cookie.value) cookie.value = cookie_val;
  } catch (err) {
    data.value = { error: err };
  }
}

async function deleteFolder(id: number) {
  const confirmed = window.confirm(
    "Are you sure you wish to delete the folder? Any remaining galleries in this folder will be moved to Unsorted",
  );
  if (!confirmed) return;
  try {
    await ses.delete_folder(id);
    data.value = await ses.folder_info();
    const cookie_val = await ses.cookie();
    if (cookie_val != cookie.value) cookie.value = cookie_val;
  } catch (err) {
    data.value = { error: err };
  }
}

const range = computed(() =>
  data.value && !("error" in data.value)
    ? Array.from({ length: data.value.length }, (_, i) => i + 1)
    : [],
);
</script>

<template>
  <div>
    <NavBar />
    <SubNavBar :nav-items="uploadNavItems" bold="Manage Folders" />
    <div v-if="data && !('error' in data)" id="o">
      <form @submit.prevent="">
        <table id="t">
          <tbody>
            <tr>
              <th>Folder Name</th>
              <th id="h2">Display Order</th>
              <th id="h3" />
            </tr>

            <tr v-for="(item, i) in data" :key="i">
              <td style="text-align: right; padding-right: 15px">
                <input
                  type="text"
                  name="fn1"
                  :value="item.name"
                  size="50"
                  maxsize="50"
                />
              </td>
              <td>
                <select
                  v-model="item.index"
                  style="width: 80px; font-size: 8pt"
                >
                  <option v-for="num in range" :key="num" :value="num">
                    {{ num }}
                  </option>
                </select>
              </td>
              <td style="padding-top: 3px">
                [<a @click="deleteFolder(item.id)">Delete</a>]
              </td>
            </tr>
            <tr>
              <td class="r">
                <input
                  type="text"
                  name="fname"
                  size="50"
                  maxsize="50"
                  placeholder="New folder name"
                  v-model="newFolderName"
                />
              </td>
              <td colspan="2">
                <input
                  type="submit"
                  value="Create Folder"
                  @click="createFolder"
                  style="width: 130px"
                />
              </td>
            </tr>
            <tr>
              <td class="r">
                <input
                  @click="autoReorder"
                  type="submit"
                  value="Save and Auto-Reorder"
                />
              </td>
              <td colspan="2">
                <input
                  type="submit"
                  @click="reorderFolders"
                  value="Save Changes"
                  style="width: 130px"
                />
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

div#o {
  margin: auto;
  width: 500px;
}

div#o p {
  padding-top: 10px;
}

table#t {
  padding-top: 10px;
  margin: auto;
}

table#t td {
  text-align: left;
}

th#h2 {
  width: 80px;
}

th#h3 {
  width: 40px;
}

td.r {
  text-align: right !important;
  padding-right: 15px;
}

input {
  font-size: 9pt !important;
}

select {
  font-size: 9pt !important;
}
</style>
