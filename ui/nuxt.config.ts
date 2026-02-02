// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
    compatibilityDate: "2025-07-15",
    vite: {
        plugins: [require("vite-plugin-wasm")()],
    },
    ssr: true,
    nitro: {
        preset: "node-server", // or 'vercel', 'cloudflare', etc.
    },
    eslint: {},
    devtools: {enabled: true},
    modules: ["@nuxt/eslint", "@nuxt/fonts", "@nuxt/icon", "@nuxt/image"],
});
