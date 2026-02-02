<script setup lang="ts">
const props = defineProps<{
  idd: bigint;
  token: string;
  favorite: number | null;
}>();
const favorite = ref(props.favorite);

watch(
  () => props.idd,
  (n, o) => {
    favorite.value = props.favorite;
  },
);
const ses = getSession();
const cookie = useCookie("EX_COOKIE");

const emit = defineEmits<{
  (e: "close", payload: undefined): void;
}>();

const tagField = defineModel<string>("tagField");

async function setFav(e: PointerEvent, index: number) {
  favorite.value = index;
  await ses.addFavorite(props.idd, props.token, index, tagField.value ?? "");
  const cookie_val = await ses.cookie();
  if (cookie_val != cookie.value) cookie.value = cookie_val;
  await fetch(`/save?id=${props.idd}&unit=${index}&task=fav`);
  if (e.shiftKey) emit("close", undefined); //window.open("", "_self").close();
  tagField.value = "";

}

async function removeFav() {
  favorite.value = null;

  await ses.removeFavorite(props.idd, props.token);
  await fetch(`/save?id=${props.idd}&unit=999&task=fav`);
  const cookie_val = await ses.cookie();
  if (cookie_val != cookie.value) cookie.value = cookie_val;
}
</script>

<template>
  <div id="gd5">
    <div
      class="button-container"
      style="
        display: flex;
        flex-direction: column;
        height: 100%;
        gap: 5px;

        width: 25px;
      "
    >
      <button
        v-for="i in 10"
        :key="i"
        :class="{ active: i - 1 == favorite }"
        @click="(e) => (i - 1 === favorite ? removeFav() : setFav(e, i - 1))"
      >
        {{ i - 1 }}
      </button>
    </div>
    <!-- TODO: actions -->
  </div>
</template>

<style scoped>
button {
  padding: 2px 0;
  font-size: 12px;
  border: none;
  border-radius: 8px;
  background-color: #e0e0e0;
  color: #333;
  cursor: pointer;
  transition: all 0.2s ease;
  width: 100%;
}

.active {
  background: #6366f1;
  color: #ffffff;
}
</style>
