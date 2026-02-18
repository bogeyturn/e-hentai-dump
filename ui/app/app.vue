<template>
  <div>
    <NuxtPage />
  </div>
</template>

<script setup>
import { useHead } from "#app";

const route = useRoute();
const updateLinks = () => {
  const links = [];
  if (route.path.startsWith("/lofi")) {
    links.push({
      rel: "stylesheet",
      href: "/z/0381/l.css",
    });
  } else {
    links.push({
      rel: "stylesheet",
      href: "/z/0381/g.css",
      media: "(prefers-color-scheme: light)",
    });
    links.push({
      rel: "stylesheet",
      href: "/z/0381/x.css",
      media: "(prefers-color-scheme: dark)",
    });
  }
  useHead({
    link: [
      ...links,
      {
        rel: "alternate",
        type: "application/atom+xml",
        title: "E-Hentai Galleries Content Feed",
        href: "https://e-hentai.org/rss/ehg.xml",
      },
      {
        rel: "alternate",
        type: "application/atom+xml",
        title: "EHTracker Torrent Feed",
        href: "https://e-hentai.org/rss/ehtracker.xml",
      },
      {
        rel: "search",
        type: "application/opensearchdescription+xml",
        title: "E-Hentai Galleries Search",
        href: "/opensearch.xml",
      },

      {
        rel: "stylesheet",
        href: "/variables.css",
      },
    ],
    meta: [
      { name: "viewport", content: "width=device-width, initial-scale=1" },
      { name: "RATING", content: "RTA-5042-1996-1400-1577-RTA" },
      {
        name: "description",
        content:
          "With more than a million absolutely free hentai doujinshi, manga, cosplay and CG galleries, E-Hentai Galleries is the world's largest free Hentai archive.",
      },
    ],
  });
};

updateLinks();
watch(() => route.path, updateLinks);
</script>
