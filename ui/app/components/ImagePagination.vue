<script setup>
import { computed } from "vue";

const props = defineProps({
  totalItems: { type: Number, required: true },
  currentPage: { type: Number, required: true }, // 1-based page index
  showCounter: { type: Boolean, default: false },
  pageSize: { type: Number, default: 20 },
  top: { type: Boolean, default: true },
});

const emit = defineEmits(["update:page"]);

const totalPages = computed(() => Math.ceil(props.totalItems / props.pageSize));

function goToPage(page) {
  if (page >= 1 && page <= totalPages.value && page !== props.currentPage) {
    emit("update:page", page);
  }
}

const pages = computed(() => {
  const total = totalPages.value;
  const current = props.currentPage;
  const maxItems = 9;
  const result = new Set();

  result.add(1);
  result.add(total);
  result.add(current);

  // Try adding pages around current
  let left = current - 1;
  let right = current + 1;

  while (result.size < maxItems && (left > 1 || right < total)) {
    if (left > 1) {
      result.add(left);
      left--;
    }
    if (result.size < maxItems && right < total) {
      result.add(right);
      right++;
    }
  }

  const sorted = Array.from(result).sort((a, b) => a - b);

  const finalPages = [];
  for (let i = 0; i < sorted.length; i++) {
    finalPages.push(sorted[i]);
    if (i < sorted.length - 1) {
      const gap = sorted[i + 1] - sorted[i];
      if (gap === 2) {
        finalPages.push(sorted[i] + 1); // single number
      } else if (gap > 2) {
        finalPages.push(".."); // multiple numbers
      }
    }
  }

  return finalPages;
});
</script>

<template>
  <div class="gtb" style="margin: 0">
    <p v-if="showCounter && totalItems" class="gpc">
      Showing {{ (currentPage - 1) * pageSize + 1 }} -
      {{ Math.min(currentPage * pageSize, totalItems) }}
      of {{ totalItems }} images
    </p>

    <table
      :class="[top ? 'ptt' : 'ptb']"
      :style="{ margin: top ? '2px auto 0' : '0 auto 10px' }"
    >
      <tbody>
        <tr>
          <!-- Previous -->
          <td class="ptdd" @click="goToPage(currentPage - 1)">
            <a :class="{ disabled: currentPage === 1 }"> &lt; </a>
          </td>

          <!-- Page numbers -->
          <td
            v-for="(page, i) in pages"
            :key="`${i}-${showCounter}`"
            :class="[{ ptds: currentPage === page }]"
            @click="goToPage(page)"
          >
            <span v-if="page === '..'">..</span>
            <a v-else>
              {{ page }}
            </a>
          </td>

          <!-- Next -->
          <td class="ptdd" @click="goToPage(currentPage + 1)">
            <a :class="{ disabled: currentPage === totalPages }"> &gt; </a>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<style scoped>
button {
  all: unset;
  cursor: pointer;
  color: #dddddd;
}
</style>
