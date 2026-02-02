import {WasmSession} from "exx";
import {getCookie} from "~/utils/cookie";

export const getSecret = () => getCookie("EX_COOKIE") ?? "";
let sessionInstance: WasmSession | null = null;

export function getSession() {
    if (!sessionInstance) {
        sessionInstance = new WasmSession(
            getSecret(),
            "http://localhost:3000/proxy/?url={url}&cookie={cookie}"
        );
    }
    return sessionInstance;
}
