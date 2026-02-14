import {WasmSession} from "exx";
import {getCookie} from "~/utils/cookie";

export const getSecret = () => getCookie("EX_COOKIE") ?? "";
let sessionInstance: WasmSession | null = null;

function backendBase(): string {
    if (import.meta.server) return "http://127.0.0.1:8081";

    return `http://${window.location.hostname}:8081`;
}

export function getSession() {
    if (!sessionInstance) {
        sessionInstance = new WasmSession(
            getSecret(),
            "http://localhost:3000/proxy/?url={url}&cookie={cookie}",
            backendBase(),
        );
    }
    return sessionInstance;
}
