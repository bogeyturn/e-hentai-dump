<template>
  <div class="gl1t" data-new="1">
    <NuxtLink :to="`/g/${idd}/${secret}/`">
      <div class="gl4t glname glink">{{ title }}</div>
    </NuxtLink>
    <div class="gl3t">
      <NuxtLink :to="`/g/${idd}/${secret}/`">
        <img :alt="title" :title="title" :src="cover_href" />
      </NuxtLink>
    </div>
    <div class="gl5t">
      <div>
        <Category :category="category" class="cs" />
        <DateComponent
          :date="date"
          :disowned="disowned"
          :new="newItem"
          :fav="fav"
        />
      </div>
      <div>
        <Stars
          :star="stars ?? 0"
          :color="computeColor(voted, stars ?? 0)"
          :opacity="1"
        />
        <div>{{ pages }} pages</div>
        <div class="gldown" />
      </div>
    </div>
  </div>
</template>
<script lang="ts" setup>
import { computed } from "vue";
import { wrapUrl } from "~/utils/wrap_url";
import DateComponent from "~/components/views/DateComponent.vue";
import Category from "~/components/views/Category.vue";
import Stars from "~/components/views/Stars.vue";

const colorsCookie = useCookie("star-setting", { default: () => "RRRRGGGGBB" });

function computeColor(voted: boolean, stars: number) {
  if (!voted) return "";
  if (!stars) return "";

  let colors = String(colorsCookie.value).toLowerCase();
  if (colors.length === 5) {
    colors = colors
      .split("")
      .map((c) => c + c)
      .join("");
  }
  const idx = Math.max(0, Math.min(stars - 1, colors.length - 1));

  return colors[idx];
}

const props = defineProps<{idd: number, secret: string, title: string, cover: string, category: string, date: {secs: number}, voted: boolean, stars: number, pages: number, fav: number | null, disowned: boolean, newItem: boolean}>();

const cover_href = computed(() => {
  return wrapUrl(props.cover);
});
</script>

<style lang="css" src="~/styles/index.css" scoped></style>
