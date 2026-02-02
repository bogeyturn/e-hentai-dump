import {WasmSession} from "exx";

export default defineNuxtPlugin(() => {
    const sessionInstance = new WasmSession(
        getSecret(),
        "http://localhost:3000/proxy/?url={url}&cookie={cookie}",
        (key: string, value: unknown) => {
            if (key === "set-cookie") {
                /*const cookie = useCookie("EX_COOKIE");
                if (typeof value === "string") {
                    const cookies = parseCookies(cookie.value ?? "")
                    const newcookies = parseCookies(value)
                    for (const key in newcookies) {
                        cookies[key] = newcookies[key]!;
                    }
                    cookie.value = stringifyCookies(cookies);
                } else {
                    throw new Error("Cookie must be a string");
                } */
            }
        }
    );

    return {
        provide: {
            session: sessionInstance
        }
    }
});