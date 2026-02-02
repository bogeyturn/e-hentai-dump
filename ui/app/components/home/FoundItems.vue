<template>
  <div class="searchtext">
    <p>
      {{ countDisplay }}
    </p>
  </div>
</template>

<script setup>
let props = defineProps({
  text: {
    type: String,
    required: true,
  },
});

const regex = /\b(\d{1,3}(?:,\d{3})*\+?)\s+results\b/;
const countDisplay = computed(() => {
  const part = props.text.match(regex);
  if (part == null) return props.text;
  const number = parseInt(part[1].replaceAll(",", ""), 10);
  const pages = number / 25;
  return props.text + " (" + Math.ceil(pages) + " pages)";
});
</script>
