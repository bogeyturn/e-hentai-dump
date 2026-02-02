export function wrapUrl(url: string) {
  const cookie = useCookie("EX_COOKIE");
  if (!cookie) {
    return url;
  }

  if (url && url.startsWith("https://s.exhentai.org/")) {
    return `/proxy/?url=${encodeURIComponent(url)}`;
  }
  return url;
}
