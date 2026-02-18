import { WasmSession } from "exx";
import { getCookie } from "~/utils/cookie";

export const getSecret = () => getCookie("EX_COOKIE") ?? "";
let sessionInstance: WasmSession | null = null;

function backendBase(): string {
  if (import.meta.server) return "http://127.0.0.1:8081";

  return `${window.location.origin}/proxy/local-api/`;
}

const proxyIp = () =>
  import.meta.server
    ? "http://127.0.0.1:3000"
    : window.location.origin;

export function getSession() {
  if (!sessionInstance) {
    sessionInstance = new WasmSession(
      getSecret(),
      `${proxyIp()}/proxy/?url={url}&cookie={cookie}`,
      backendBase(),
    );
  }
  return sessionInstance;
}
