<template>
  <div id="nb" class="nosel">
    <div v-for="link in links" :key="link.text">
      <NuxtLink :to="link.url">{{ link.text }}</NuxtLink>
    </div>
  </div>
</template>

<script setup>
const links = [
  { text: "Front Page", url: "/" },
  { text: "Watched", url: "/watched" },
  { text: "Popular", url: "/popular" },
  { text: "Torrents", url: "/torrents" },
  { text: "Favorites", url: "/favorites" },
  { text: "My Home", url: "/home" },
  { text: "My Uploads", url: "/upload/manage" },
  { text: "Toplists", url: "/toplist" },
  { text: "Bounties", url: "/bounty" },
  { text: "News", url: "/news" },
  { text: "Forums", url: "https://forums.e-hentai.org/" },
  { text: "Wiki", url: "https://ehwiki.org/" },
  { text: "HentaiVerse", url: "https://hentaiverse.org/" },
];
</script>
