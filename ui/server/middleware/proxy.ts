import { defineEventHandler, getQuery, proxyRequest } from "h3";
import { URL } from "node:url";

export default defineEventHandler(async (event) => {
  const req = event.node.req;
  const url_ = req.url || "";
  if (!url_.startsWith("/proxy/")) return;

  const { url } = getQuery(event);
  const cookie = getCookie(event, "EX_COOKIE");

  const decodedCookie = cookie ? decodeURIComponent(String(cookie)) : undefined;
  if (!url || typeof url !== "string")
    return { error: "Missing ?url parameter" };

  let target: URL;
  try {
    target = new URL(url);
  } catch {
    return { error: "Invalid URL" };
  }

  // Only cache GET requests
  if (req.method !== "GET") {
    return proxyRequest(event, target.toString(), {
      target: target.origin,
      changeOrigin: true,
      headers: {
        origin: target.origin,
        "custom-header": "proxy",
        referer: target.origin,
        ...(decodedCookie ? { cookie: String(decodedCookie) } : {}),
      },
      onProxyRes(proxyRes: any) {
        delete proxyRes.headers["access-control-allow-origin"];
        delete proxyRes.headers["access-control-allow-credentials"];
      },
    });
  }

  const response = await proxyRequest(event, target.toString(), {
    target: target.origin,
    changeOrigin: true,
    headers: {
      origin: target.origin,
      "custom-header": "proxy",
      referer: target.origin,
      ...(decodedCookie ? { cookie: String(decodedCookie) } : {}),
    },
    async onProxyRes(proxyRes: any) {
      delete proxyRes.headers["access-control-allow-origin"];
      delete proxyRes.headers["access-control-allow-credentials"];
    },
  });

  return response;
});
