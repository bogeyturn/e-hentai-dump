export default defineNuxtRouteMiddleware((to) => {
  const phpExtension = /\.php$/;

  if (phpExtension.test(to.path)) {
    return navigateTo(
      {
        path: to.path.replace(phpExtension, ""),
        query: to.query,
      },
      { redirectCode: 301 },
    );
  }
});
