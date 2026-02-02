<template>
  <div>
    <form @submit.prevent="handleSubmit">
      <input
        v-model="cookieValue"
        type="text"
        placeholder="Enter cookie value"
      />
      <button type="submit">Set Cookie</button>
    </form>
  </div>
</template>

<script setup>
const cookie = useCookie("EX_COOKIE");
const cookieValue = ref(cookie.value ?? "");

function handleSubmit() {
  const trimmed = String(cookieValue.value).trim();
  if (trimmed) {
    cookie.value = trimmed;
    navigateTo("/");
  }
}
</script>
