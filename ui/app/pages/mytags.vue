<script setup lang="ts">
import { configNavItems } from "~/utils/config";
import NavBar from "~/components/NavBar.vue";
import SubNavBar from "~/components/SubNavBar.vue";
</script>
<template>
  <div>
    <NavBar />
    <SubNavBar :nav-items="configNavItems" bold="My Tags" />
  </div>
</template>
