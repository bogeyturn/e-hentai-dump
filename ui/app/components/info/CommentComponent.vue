<script setup lang="ts">
import type { Comment } from "exx";

defineProps<{ comment: Comment }>();
const showPermanent = ref(false);
const showVotes = ref(false);
</script>

<template>
  <div class="c2">
    <div class="c3">
      {{ comment.posted }} by: &nbsp;
      <NuxtLink to="/">{{ comment.uploader }}</NuxtLink>
    </div>
    <div v-if="comment.id !== 0" class="c4 nosel">
      [<a>Vote+</a>] &nbsp; [<a>Vote-</a>]
    </div>
    <div v-if="comment.id === 0" class="c4 nosel">
      <a name="ulcomment" />Uploader Comment
    </div>
    <div
      v-if="comment.id !== 0"
      class="c5 nosel"
      @click="showPermanent = !(showPermanent ?? false)"
      @mouseenter="showVotes = true"
      @mouseleave="showVotes = false"
    >
      Score
      <span style="opacity: 1"
        >{{ (comment.votes ?? 0) >= 0 ? "+" : "" }}{{ comment.votes }}</span
      >
    </div>
    <div class="c" />
  </div>

  <div class="c6" v-html="comment.text" />
  <div v-show="showVotes || showPermanent" class="c7" style="display: none">
    <span v-for="(vote, i) in comment.voters" :key="i">
      {{ vote }}
    </span>
  </div>
</template>

<style scoped></style>
