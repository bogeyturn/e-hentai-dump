<template>
  <div id="toppane">
    <div id="searchbox" class="idi">
      <form style="margin: 0; padding: 0" @submit.prevent="onSubmit">
        <table class="itc">
          <tbody>
            <tr v-for="(row, rowIndex) in categoryRows" :key="rowIndex">
              <td v-for="cat in row" :key="cat.id">
                <div
                  :id="`cat_${cat.id}`"
                  class="cs"
                  :class="cat.class"
                  :data-disabled="
                    selectedCategories.indexOf(cat.id) !== -1 ? '1' : null
                  "
                  @click="toggleCategory(cat.id)"
                >
                  {{ cat.label }}
                </div>
              </td>
            </tr>
          </tbody>
        </table>

        <div>
          <input
            id="f_search"
            v-model="tempQuery"
            type="text"
            name="f_search"
            placeholder="Search Keywords"
            size="90"
            maxlength="200"
          />
          <input type="submit" value="Search" />
          <input type="button" value="Clear" @click="clearSearch" />
        </div>

        <div>
          [
          <a href="#" @click.prevent="toggleAdvanced">
            {{ showAdvanced ? "Hide" : "Show" }} Advanced Options
          </a>
          ] &nbsp; &nbsp;
          <template v-if="fileSearch"
            >[
            <a href="#" @click.prevent="toggleFileSearch">
              {{ showfileSearch ? "Hide" : "Show" }} File Search
            </a>
            ]
          </template>
        </div>

        <div v-if="showAdvanced" id="advdiv">
          <div class="searchadv">
            <div>
              <div>
                <label class="lc"
                  ><input type="checkbox" v-model="showuser" /><span></span
                  >Include your Galleries</label
                >
              </div>
              <div>
                <label class="lc"
                  ><input type="checkbox" v-model="showfav" /><span></span
                  >Include fav Galleries</label
                >
              </div>
            </div>
            <div>
              <div>
                <label class="lc"
                  ><input
                    type="checkbox"
                    v-model="browseExpunged"
                    name="f_sh"
                  /><span></span> Browse Expunged Galleries</label
                >
              </div>
              <div>
                <label class="lc"
                  ><input
                    type="checkbox"
                    v-model="requireTorrent"
                    name="f_sto"
                  /><span></span> Require Gallery Torrent</label
                >
              </div>
            </div>
            <div>
              <div>
                Between
                <input
                  v-model="minPages"
                  type="text"
                  id="f_spf"
                  name="f_spf"
                  size="4"
                  maxlength="4"
                  style="width: 30px"
                />
                and
                <input
                  v-model="maxPages"
                  type="text"
                  id="f_spt"
                  name="f_spt"
                  size="4"
                  maxlength="4"
                  style="width: 30px"
                />
                pages
              </div>
              <div>
                Minimum Rating:
                <select id="f_srdd" name="f_srdd" v-model="minRating">
                  <option value="0">Any Rating</option>
                  <option value="2">2 Stars</option>
                  <option value="3">3 Stars</option>
                  <option value="4">4 Stars</option>
                  <option value="5">5 Stars</option>
                </select>
              </div>
            </div>
            <div>
              <div>Disable custom filters for:</div>
              <div>
                <label class="lc"
                  ><input
                    type="checkbox"
                    v-model="disableLang"
                    name="f_sfl"
                  /><span></span> Language</label
                >
              </div>
              <div>
                <label class="lc"
                  ><input
                    type="checkbox"
                    v-model="disableUploader"
                    name="f_sfu"
                  /><span></span> Uploader</label
                >
              </div>
              <div>
                <label class="lc"
                  ><input
                    type="checkbox"
                    v-model="disableTags"
                    name="f_sft"
                  /><span></span> Tags</label
                >
              </div>
            </div>
          </div>
        </div>
      </form>
    </div>
    <div v-if="showfileSearch" id="fsdiv" class="idi" style="margin-top: 10px">
      <form>
        <div>
          Select a file to upload, then hit File Search. All public galleries
          containing this exact file will be displayed.
        </div>
        <div>
          <input type="file" name="sfile" size="40" />
          <input type="submit" name="f_sfile" value="File Search" />
        </div>
        <div>
          For color images, the system can also perform a similarity lookup to
          find resampled images.
        </div>
        <div class="searchadv">
          <div>
            <div>
              <label class="lc"
                ><input
                  type="checkbox"
                  name="fs_similar"
                  checked="checked"
                /><span></span> Use Similarity Scan</label
              >
            </div>
            <div>
              <label class="lc"
                ><input type="checkbox" name="fs_covers" /><span></span> Only
                Search Covers</label
              >
            </div>
          </div>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from "vue";
import { updateUrl } from "~/utils/routing";

const route = useRoute();
const showfav = defineModel<boolean>("showfav");
const showuser = defineModel<boolean>("showuser");

const props = defineProps({
  fileSearch: {
    type: Boolean,
    default: true,
  },
});

const searchQuery = computed(() =>
  Array.isArray(route.query.f_search)
    ? (route.query.f_search[0] ?? "")
    : (route.query.f_search ?? ""),
);

const tempQuery = ref<String>(searchQuery.value);
const router = useRouter();

const selectedCategories = ref<Array<Number>>([]);

function restoreCategoriesFromUrl() {
  const catParam = route.query.f_cats;
  let catMask = 0;
  if (Array.isArray(catParam)) {
    catMask = Number(catParam[0]) || 0;
  } else if (catParam) {
    catMask = Number(catParam) || 0;
  }

  selectedCategories.value = categories
    .filter((c) => (catMask & c.id) !== 0)
    .map((c) => c.id);
}

const showAdvanced = ref(false);
const showfileSearch = ref(false);

const categories = [
  { id: 2, label: "Doujinshi", class: "ct2" },
  { id: 4, label: "Manga", class: "ct3" },
  { id: 8, label: "Artist CG", class: "ct4" },
  { id: 16, label: "Game CG", class: "ct5" },
  { id: 512, label: "Western", class: "cta" },
  { id: 256, label: "Non-H", class: "ct9" },
  { id: 32, label: "Image Set", class: "ct6" },
  { id: 64, label: "Cosplay", class: "ct7" },
  { id: 128, label: "Asian Porn", class: "ct8" },
  { id: 1, label: "Misc", class: "ct1" },
];
restoreCategoriesFromUrl();

const categoryRows = computed(() => {
  return [categories.slice(0, 5), categories.slice(5)];
});

function toggleCategory(id: number) {
  const index = selectedCategories.value.indexOf(id);
  if (index === -1) {
    selectedCategories.value.push(id);
  } else {
    selectedCategories.value.splice(index, 1);
  }
}

function toggleAdvanced() {
  showAdvanced.value = !showAdvanced.value;
}

function toggleFileSearch() {
  showfileSearch.value = !showfileSearch.value;
}

watch(searchQuery, (newVal) => {
  tempQuery.value = newVal ?? "";
});

function clearSearch() {
  updateUrl(router, { f_search: "", prev: null, next: null });
  selectedCategories.value = [];
  showAdvanced.value = false;
}

const browseExpunged = ref(false);
const requireTorrent = ref(false);
const disableLang = ref(false);
const disableUploader = ref(false);
const disableTags = ref(false);

const minPages = ref("");
const maxPages = ref("");
const minRating = ref("0");
const includeGalleries = useCookie("includeGalleries");
const includeFavGalleries = useCookie("includeFavGalleries");
watch(showuser, (newVal) => {
  includeGalleries.value = newVal ? "1" : "0";
});

watch(showfav, (newVal) => {
  includeFavGalleries.value = newVal ? "1" : "0";
});

function onSubmit() {
  const s = tempQuery.value;
  let cat: number | null | string = selectedCategories.value.reduce(
    (mask, id) => mask | id,
    0,
  );
  if (cat == 0 || cat == "0") {
    cat = null;
  }

  updateUrl(router, {
    f_search: s,
    prev: null,
    next: null,
    cat: cat,
    minRating: parseInt(minRating.value) ? minRating.value : null,
    maxPages: maxPages.value ? maxPages.value : null,
    minPages: minPages.value ? minPages.value : null,
    disableTags: disableTags.value ? true : null,
    disableUploader: disableUploader.value ? true : null,
    disableLang: disableLang.value ? true : null,
    requireTorrent: requireTorrent.value ? true : null,
    browseExpunged: browseExpunged.value ? true : null,
  });
}

onMounted(() => {
  browseExpunged.value = route.query.f_sh === "1";
  requireTorrent.value = route.query.f_sto === "1";
  disableLang.value = route.query.f_sfl === "1";
  disableUploader.value = route.query.f_sfu === "1";
  disableTags.value = route.query.f_sft === "1";

  minPages.value = route.query.f_spf || "";
  maxPages.value = route.query.f_spt || "";
  minRating.value = route.query.f_srdd || "0";
});
</script>
