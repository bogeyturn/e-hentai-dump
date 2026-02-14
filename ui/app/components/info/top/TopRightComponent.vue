<script setup lang="ts">
const props = defineProps<{
  idd: bigint;
  token: string;
  favorite: number | null;
}>();
const favorite = ref(props.favorite);

watch(
    () => props.idd,
    (_n: bigint, _o: bigint) => {
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
  await ses.addFavorite(props.idd, props.token, index, tagField.value ?? "", true);
  await ses.addFavorite(props.idd, props.token, index, tagField.value ?? "", false);
  const cookie_val = await ses.cookie();
  if (cookie_val != cookie.value) cookie.value = cookie_val;
  if (e.shiftKey) emit("close", undefined); //window.open("", "_self").close();
  tagField.value = "";

}

async function removeFav() {
  favorite.value = null;
  await ses.removeFavorite(props.idd, props.token, true);
  await ses.removeFavorite(props.idd, props.token, false);
  const cookie_val = await ses.cookie();
  if (cookie_val != cookie.value) cookie.value = cookie_val;
}
</script>

<template>
  <div class="button-grid">
    <button
        v-for="i in 10"
        :key="i"

        :class="{ active: i - 1 === favorite, b: i > 5}"
        @click="(e) => (i - 1 === favorite ? removeFav() : setFav(e, i - 1))"
    >
      {{ i - 1 }}
    </button>
  </div>
</template>

<style scoped>
.button-grid {
  display: grid;
  grid-template-columns: repeat(5, 1fr);
  grid-auto-rows: 1fr;
  gap: 0;
  border: 1px solid #000000;
  border-bottom: none;
  width: fit-content;
}

button {
  padding: 2px 0;
  font-size: 12px;
  border: none;
  border-radius: 0;
  background-color: transparent;
  color: white;
  cursor: pointer;
  transition: all 0.2s ease;

  width: 25px;
  height: 25px;
}

.active {
  background: #6366f1;
  color: #ffffff;
}
.b.active {
  border-bottom: 0.5px solid #000000;
}
</style>