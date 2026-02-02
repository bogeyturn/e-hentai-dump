export default defineNuxtRouteMiddleware((to) => {
    const uploaderPattern = /^\/uploader\/(.+)$/;

    const match = to.path.match(uploaderPattern);
    if (match) {
        const name = match[1];
        return navigateTo(
            {
                path: "/",
                query: {
                    ...to.query,
                    f_search: `uploader:${name}`,
                },
            },
            {redirectCode: 301},
        );
    }
});