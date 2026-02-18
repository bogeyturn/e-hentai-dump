import { defineEventHandler, getQuery, proxyRequest } from "h3";
import { URL } from "node:url";

const LOCAL_API_ORIGIN = process.env.LOCAL_API_ORIGIN ?? "http://127.0.0.1:8081";
type ProxyResLike = {
  headers: Record<string, string | string[] | undefined>;
};

export default defineEventHandler(async (event) => {
  const req = event.node.req;
  const url_ = req.url || "";
  if (!url_.startsWith("/proxy/")) return;

  if (url_.startsWith("/proxy/local-api/")) {
    const parsed = new URL(url_, "http://localhost");
    const path = parsed.pathname.replace("/proxy/local-api", "");
    const targetUrl = `${LOCAL_API_ORIGIN}${path}${parsed.search}`;

    return proxyRequest(event, targetUrl, {
      target: LOCAL_API_ORIGIN,
      changeOrigin: true,
    });
  }

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
      onProxyRes(proxyRes: ProxyResLike) {
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
    async onProxyRes(proxyRes: ProxyResLike) {
      delete proxyRes.headers["access-control-allow-origin"];
      delete proxyRes.headers["access-control-allow-credentials"];
    },
  });

  return response;
});
