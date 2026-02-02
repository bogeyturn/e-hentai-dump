<script setup lang="ts">
import type { Comment } from "exx";
import { ref } from "vue";
import CommentComponent from "~/components/info/CommentComponent.vue";

defineProps<{ comments: Comment[] }>();

const show_form = ref(false);
</script>

<template>
  <a name="comments" />
  <div id="cdiv" class="gm">
    <div v-for="comment in comments" :key="comment.id" class="c1">
      <CommentComponent :comment="comment" />
    </div>

    <div v-if="!show_form" id="chd">
      <p id="postnewcomment">
        [<a style="cursor: pointer" @click="() => (show_form = true)"
          >Post New Comment</a
        >]
      </p>
    </div>
    <a name="cnew" />
    <div v-if="show_form" id="formdiv">
      <form @submit.prevent="">
        <textarea
          name="commenttext_new"
          placeholder="Enter your comments here, then hit Post Comment. If the last comment posted is yours, this will be appended to that post."
        />
        <p><input type="submit" value="Post Comment" /></p>
      </form>
    </div>
  </div>
</template>

<style scoped></style>
