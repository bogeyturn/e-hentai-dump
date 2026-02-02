<script setup lang="ts">
import CommonView from "~/components/views/CommonView.vue";
import type { Pid } from "exx";

const ses = getSession();

const route = useRoute();

const props = defineProps<{
  favcat?: number;
  next: string;
  prev: string;
  query: string;
  orderByPublished: boolean;
}>();
const new_search = ref(props.query);

const cookie = useCookie("EX_COOKIE");
const { data } = await useAsyncData(
  () =>
    `favorites-${props.favcat}-${props.next}-${props.prev}-${props.query}-${props.orderByPublished}`,
  async () => {
    try {
      const v = await ses.list_favorite({
        query: props.query,
        pid: parsePid(props.next ? props.next : props.prev ? props.prev : null),
        forward: !!props.next,
        cat: props.favcat == undefined ? null : props.favcat,
        order_by_published: props.orderByPublished,
      });
      const cookie_val = await ses.cookie();
      if (cookie_val != cookie.value) cookie.value = cookie_val;
      return v;
    } catch (err) {
      return { error: err };
    }
  },
  { server: true, immediate: true },
);

function parsePid(s: string | null): Pid | null {
  if (!s) return null;
  if (s.includes("-")) {
    const ids = s.split("-", 2);
    return { Range: [parseInt(ids[0]!), parseInt(ids[1]!)] };
  } else {
    return { Single: parseInt(s) };
  }
}

function displayPid(pid: Pid | null) {
  if (!pid) return null;
  if ("Single" in pid) return pid.Single;
  if ("Range" in pid) return `${pid.Range[0]}-${pid.Range[1]}`;
  return null;
}

const view = useCookie("search-view", {
  default: () => "t",
});
</script>

<template>
  <div
    v-if="data && !('error' in data)"
    class="ido"
    :style="'min-width:930px;' + (view == 't' ? 'max-width: 1370px' : '')"
  >
    <h1
      style="
        font-size: 10pt;
        font-weight: bold;
        margin: 3px;
        text-align: center;
      "
    >
      Favorites
    </h1>

    <div
      class="nosel"
      style="
        font-size: 8pt;
        position: relative;
        width: 825px;
        margin: 10px auto 5px;
      "
    >
      <NuxtLink
        v-for="(count, i) in data.counts"
        :key="i"
        :to="{
          query: {
            ...route.query,
            favcat: i,
            next: undefined,
            prev: undefined,
          },
        }"
      >
        <div
          class="fp"
          :class="favcat == i ? 'fps' : ''"
          style="width: 160px; padding: 2px 0 0; float: left"
        >
          <div
            style="
              font-weight: bold;
              float: left;
              text-align: right;
              width: 30px;
              height: 20px;
              padding: 2px 3px 0 0;
            "
          >
            {{ count }}
          </div>
          <div
            class="i"
            :style="`background-image:url(/g/fav.png); background-position:0px -${2 + 19 * i}px; position:relative; top:1px`"
            :title="`Favorites ${i}`"
          />
          <div
            style="
              float: left;
              text-align: left;
              height: 20px;
              padding: 2px 0 0 3px;
            "
          >
            Favorites {{ i }}
          </div>
        </div>
      </NuxtLink>

      <div style="clear: both" />
      <NuxtLink to="/favorites" style="text-decoration: none">
        <div
          class="fp"
          :class="favcat == undefined ? 'fps' : ''"
          style="
            width: 140px;
            padding: 4px 0 0;
            margin: 5px auto 0;
            text-align: center;
            font-weight: bold;
            position: relative;
            left: -8px;
          "
        >
          Show All Favorites
        </div>
      </NuxtLink>
    </div>

    <div style="width: 825px; margin: 10px auto 5px; text-align: center">
      <form>
        <div style="margin: 3px auto 0; position: relative">
          <input
            v-model="new_search"
            type="text"
            name="f_search"
            size="90"
            maxlength="200"
            placeholder="Search Keywords"
          />
          <NuxtLink
            :to="{
              path: route.path,
              query: {
                ...route.query,
                prev: undefined,
                next: undefined,
                f_search: new_search,
              },
            }"
          >
            <input type="submit" value="Search"
          /></NuxtLink>
          <NuxtLink to="/favorites"
            ><input type="button" value="Clear"
          /></NuxtLink>
        </div>
      </form>
    </div>
    <CommonView
      :first="!prev"
      :last="displayPid(data.last)"
      :query="''"
      :prev="displayPid(data.prev)"
      :next="displayPid(data.next)"
      :items="data.items"
      :order-enabled="true"
    />
  </div>
</template>

<style scoped>
div.fp {
  height: 20px;
  cursor: pointer;
  border-radius: 9px;
  border: 1px solid transparent;
}

div.fp:hover {
  background: #f3f0e0;
  border: 1px solid #c2c1c1;
}

div.fps {
  background: #f3f0e0;
  border: 1px solid #c2c1c1;
}

@media (prefers-color-scheme: dark) {
  div.fp:hover {
    background: #43464e;
    border: 1px solid #c2c1c1;
  }

  div.fps {
    background: #43464e;
    border: 1px solid #c2c1c1;
  }
}
</style>
