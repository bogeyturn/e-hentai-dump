import { getCookie } from "@/utils/cookie";

export default defineNuxtRouteMiddleware((to) => {
  if (to.path === "/set-cookie") return;

  const exCookie = getCookie("EX_COOKIE");

  if (!exCookie) {
    return navigateTo("/set-cookie");
  }
});
