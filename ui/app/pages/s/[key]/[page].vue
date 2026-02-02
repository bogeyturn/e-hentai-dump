<script setup lang="ts">
import { useAsyncData } from "#imports";

const ses = getSession();

const route = useRoute();
const key = route.params.key as string;
const id = computed(() => String(route.params.page).split("-", 2)[0] ?? "");
const page = computed(() => String(route.params.page).split("-", 2)[1] ?? "");

const showKeyCookie = useCookie("showKey", { maxAge: 3600 });

const todo = "a";

const showKey = () => {
  if (showKeyCookie.value) {
    const [cookieId, vals2] = showKeyCookie.value.split("-", 2);
    if (cookieId != id.value) {
      showKeyCookie.value = null;
      return null;
    }
    return vals2;
  }
  return null;
};
const cookie = useCookie("EX_COOKIE");
const { data: img_data } = useAsyncData(
  () => `image-${key}-${id.value}${page.value}`,
  async () => {
    if (
      Number.isInteger(Number(id.value)) ||
      Number.isInteger(Number(page.value))
    )
      return null;

    const data = await ses.nextImg(
      BigInt(parseInt(id.value)),
      key,
      parseInt(page.value),
      showKey(),
    );
    const cookie_val = await ses.cookie();
    if (cookie_val != cookie.value) cookie.value = cookie_val;
    showKeyCookie.value = data.showkey;
    return data;
  },
  { server: true, immediate: true },
);
</script>

<template>
  <div
    v-if="img_data"
    id="i1"
    class="sni"
    style="max-width: 100%; width: 1280px"
  >
    <h1>
      [Buchiyama Melon Peach (Buchiyama Buchio)] Kyuukeichuu | 休憩中 (Blue
      Archive) [Chinese] [Digital] [千禧美少女汉化]
    </h1>
    <div id="i2">
      <div class="sn">
        <NuxtLink :to="`/s/${todo}/${id}-1`"
          ><img src="/g/f.png" alt=""
        /></NuxtLink>
        <NuxtLink
          v-if="img_data.prev"
          id="prev"
          :to="`/s/${img_data.prev}/${id}-${parseInt(page) - 1}`"
          ><img src="/g/p.png" alt=""
        /></NuxtLink>
        <img v-else src="/g/p.png" alt="" />
        <div>
          <span>{{ page }}</span> / <span>{{ img_data.count }}</span>
        </div>
        <NuxtLink
          v-if="img_data.next"
          id="next"
          :to="`/s/${img_data.next}/${id}-${parseInt(page) + 1}`"
          ><img src="/g/n.png" alt=""
        /></NuxtLink>
        <img v-else src="/g/n.png" alt="" />
        <NuxtLink :to="`/s/${todo}/${id}-${img_data.count}`"
          ><img src="/g/l.png" alt=""
        /></NuxtLink>
      </div>
      <div>
        {{ img_data.name }} :: {{ img_data.w }} x {{ img_data.h }} ::
        {{ img_data.size }}
      </div>
    </div>
    <div id="i3">
      <NuxtLink :to="`/s/${img_data.next}/${id}-${parseInt(page) + 1}`"
        ><img
          id="img"
          :src="img_data.url"
          :style="`width: 1280px; aspect-ratio: ${img_data.w} / ${img_data.h} `"
          alt=""
      /></NuxtLink>
    </div>
    <div id="i4">
      <div>
        {{ img_data.name }} :: {{ img_data.w }} x {{ img_data.h }} ::
        {{ img_data.size }}
      </div>
      <div class="sn">
        <NuxtLink :to="`/s/${todo}/${id}-1`"
          ><img src="/g/f.png" alt=""
        /></NuxtLink>
        <NuxtLink
          v-if="img_data.prev"
          id="prev"
          :to="`/s/${img_data.prev}/${id}-${parseInt(page) - 1}`"
          ><img src="/g/p.png" alt=""
        /></NuxtLink>
        <img v-else src="/g/p.png" alt="" />
        <div>
          <span>{{ page }}</span> / <span>{{ img_data.count }}</span>
        </div>
        <NuxtLink
          v-if="img_data.next"
          id="next"
          :to="`/s/${img_data.next}/${id}-${parseInt(page) + 1}`"
          ><img src="/g/n.png" alt=""
        /></NuxtLink>
        <img v-else src="/g/n.png" alt="" />
        <NuxtLink :to="`/s/${todo}/${id}-${img_data.count}`"
          ><img src="/g/l.png" alt=""
        /></NuxtLink>
      </div>
    </div>
    <div id="i5">
      <div class="sb">
        <NuxtLink :to="`/g/${id}/${img_data.gallery_key}/`"
          ><img src="/g/b.png" referrerpolicy="no-referrer" alt=""
        /></NuxtLink>
      </div>
    </div>
    <div id="i6">
      <div>
        &nbsp; <img src="/g/mr.gif" class="mr" alt="" />
        <NuxtLink :href="`/?f_shash=${img_data.hash}`"
          >Show galleries with this image
        </NuxtLink>
      </div>
      <div>
        &nbsp; <img src="/g/mr.gif" class="mr" alt="" />
        <a id="loadfail" href="#">Reload broken image</a>
      </div>
      <div>
        &nbsp; <img src="/g/mr.gif" class="mr" alt="" />
        <NuxtLink v-if="img_data.original" :to="img_data.original"
          >Download original {{ img_data.w }} x {{ img_data.h }}
          {{ img_data.size }}
        </NuxtLink>
      </div>
    </div>
  </div>
</template>

<style scoped></style>
