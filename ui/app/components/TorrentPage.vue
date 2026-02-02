<script setup lang="ts">
const ses = getSession();

const new_search = ref("");
const props = defineProps<{
  search: string;
  u?: number;
  s: "seeded" | "unseeded" | "";
  o: string;
  page: number;
}>();
const route = useRoute();

function toSort(s?: string | null) {
  if (!s || s == "a") {
    return "Date";
  } else if (s == "b") {
    return "Size";
  } else if (s == "s") {
    return "Seeds";
  } else if (s == "d") {
    return "Peers";
  } else if (s == "c") {
    return "Downloads";
  }
  return "Date";
}

const cookie = useCookie("EX_COOKIE");
const { data } = await useAsyncData(
  async () => {
    try {
      const se =
        props.s === "seeded"
          ? "Seeded"
          : props.s === "unseeded"
            ? "Unseeded"
            : "All";

      const v = await ses.torrents(
        props.search,
        props.page,
        se,
        props.u,
        toSort(props.o ? props.o[0] : null),
        props.o != null ? props.o[1] == "a" : false,
      );
      const cookie_val = await ses.cookie();
      if (cookie_val != cookie.value) cookie.value = cookie_val;
      return v;
    } catch (err) {
      return { error: err };
    }
  },
  { server: true, immediate: true },
);

function new_o(target: string) {
  if (!props.o) {
    if (target == "a") {
      return "aa";
    }
    return target + "d";
  }
  if (props.o! == "aa" && target == "a") {
    return undefined;
  }

  if (props.o!.startsWith(target)) {
    if (props.o.endsWith("a")) {
      return target + "d";
    } else {
      return target + "a";
    }
  }

  if (target == "a") {
    return undefined;
  }

  return target + "d";
}
</script>

<template>
  <div v-if="data && !('error' in data)" class="ido">
    <h1 class="ih">Torrents</h1>
    <div style="margin: 1px auto 10px; text-align: center">
      <form id="torrentform" method="get" action="">
        <input
          id="focusme"
          v-model="new_search"
          type="text"
          name="search"
          placeholder="Search Keywords"
          size="50"
          maxlength="255"
          style="font-size: 10pt; width: 500px"
        />
        <br />
        <p style="font-size: 8pt; padding: 5px 0 10px">
          Status:
          <NuxtLink
            :to="{
              path: route.path,
              query: { ...route.query, s: undefined },
            }"
            style="font-weight: bold"
            :style="s ? 'text-decoration:none' : ''"
          >
            All
          </NuxtLink>
          &nbsp;
          <NuxtLink
            :to="{
              path: route.path,
              query: { ...route.query, s: 'seeded' },
            }"
            style="font-weight: bold"
            :style="s != 'seeded' ? 'text-decoration:none' : ''"
            >Seeded
          </NuxtLink>
          &nbsp;
          <NuxtLink
            :to="{
              path: route.path,
              query: { ...route.query, s: 'unseeded' },
            }"
            style="font-weight: bold"
            :style="s != 'unseeded' ? 'text-decoration:none' : ''"
            >Unseeded
          </NuxtLink>
          &nbsp; &nbsp; &nbsp; &nbsp; Show:
          <NuxtLink
            :to="{
              path: route.path,
              query: { ...route.query, u: undefined },
            }"
            style="font-weight: bold"
            :style="u ? 'text-decoration:none' : ''"
            >All Torrents
          </NuxtLink>
          &nbsp;
          <NuxtLink
            :to="{
              path: route.path,
              query: { ...route.query, u: data.user_id },
            }"
            style="font-weight: bold"
            :style="!u ? 'text-decoration:none' : ''"
            >{{ data.name ?? "Only My Torrents" }}
          </NuxtLink>
        </p>
        <NuxtLink
          :to="{
            path: route.path,
            query: { ...route.query, search: new_search },
          }"
          ><input type="button" value="Search Torrents"
        /></NuxtLink>
        <NuxtLink to="/torrents"
          ><input type="button" value="Clear"
        /></NuxtLink>
      </form>
    </div>
    <div style="margin: auto; text-align: center" />
    <ImagePagination
      :current-page="page"
      :total-items="50 * 100"
      :page-size="100"
      :show-counter="true"
      :top="true"
    />
    <table class="itg">
      <tbody>
        <tr>
          <th style="text-align: center; width: 95px">
            <NuxtLink
              :to="{
                path: route.path,
                query: { ...route.query, o: new_o('a') },
              }"
              >Added
            </NuxtLink>
          </th>
          <th>Torrent Name</th>
          <th style="text-align: center; width: 50px">Gallery</th>
          <th style="text-align: center; width: 70px">
            <NuxtLink
              :to="{
                path: route.path,
                query: { ...route.query, o: new_o('b') },
              }"
              >Size
            </NuxtLink>
          </th>
          <th style="text-align: center; width: 35px">
            <NuxtLink
              :to="{
                path: route.path,
                query: { ...route.query, o: new_o('s') },
              }"
              >Seeds
            </NuxtLink>
          </th>
          <th style="text-align: center; width: 35px">
            <NuxtLink
              :to="{
                path: route.path,
                query: { ...route.query, o: new_o('d') },
              }"
              >Peers
            </NuxtLink>
          </th>
          <th style="text-align: center; width: 40px">
            <NuxtLink
              :to="{
                path: route.path,
                query: { ...route.query, o: new_o('c') },
              }"
              >DLs
            </NuxtLink>
          </th>
          <th style="text-align: center; width: 90px">Uploader</th>
        </tr>

        <tr v-for="(item, index) in data.items" :key="index">
          <td class="itd" style="width: 102px; white-space: nowrap">
            {{ displayDate(item.added) }}
          </td>
          <td class="itd">
            <div
              style="
                height: 15px;
                min-width: 300px;
                max-width: 600px;
                overflow: hidden;
              "
            >
              <a rel="nofollow">{{ item.name }}</a>
            </div>
          </td>
          <td class="itd" style="text-align: right; width: 50px">
            <a>{{ item.gallery }}</a>
          </td>
          <td
            class="itd"
            style="text-align: right; width: 70px; white-space: nowrap"
          >
            {{ formatBytesIB(item.size) }}
          </td>
          <td class="itd" style="text-align: right; width: 30px">
            {{ item.seeds }}
          </td>
          <td class="itd" style="text-align: right; width: 30px">
            {{ item.peers }}
          </td>
          <td class="itd" style="text-align: right; width: 40px">
            {{ item.dls }}
          </td>
          <td class="itd" style="width: 90px">
            <div style="width: 120px; overflow: hidden">
              <NuxtLink
                :to="{
                  path: route.path,
                  query: { ...route.query, u: item.uploader_id },
                }"
                >{{ item.uploader }}
              </NuxtLink>
            </div>
          </td>
        </tr>
      </tbody>
    </table>

    <ImagePagination
      :current-page="page"
      :total-items="50 * 100"
      :page-size="100"
      :show-counter="false"
      :top="false"
    />

    <div style="margin: auto; text-align: center" />
    <p class="ip">
      You cannot add torrents directly to this page. To upload torrents to this
      system, visit the torrent screen for a gallery.
    </p>
  </div>
</template>

<style scoped></style>
