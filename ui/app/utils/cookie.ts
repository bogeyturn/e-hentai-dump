export function getCookie(name: string) {
  const event = useRequestEvent();
  if (event?.node?.req) {
    const cookies = event.node.req.headers.cookie;
    if (cookies) {
      return getSpecificCookie(cookies, name);
    }
  }

  if (import.meta.client) {
    return getSpecificCookie(document.cookie, name);
  }

  return null;
}

function getSpecificCookie(data: string, name: string) {
  const cookies = data.split("; ");
  for (const cookie of cookies) {
    const [key, value] = cookie.split("=");
    if (key === name && value) return decodeURIComponent(value);
  }
  return null;
}


export function parseCookies(str:string) {
  return str.split("; ").reduce((acc:Record<string, string>, pair) => {
    const [key, value = ""] = pair.split("=");
    if (key) acc[key] = value;
    return acc;
  }, {});
}

export function stringifyCookies(obj: Record<string, string>): string {
  return Object.entries(obj)
      .map(([key, value]) => `${key}=${value}`)
      .join("; ");
}

export function setCookie(
    name: string,
    value: string,
    options?: {
      path?: string
      maxAge?: number
      expires?: Date
      sameSite?: 'lax' | 'strict' | 'none'
      secure?: boolean
    }
) {
  if (!import.meta.client) return

  let cookie = `${encodeURIComponent(name)}=${encodeURIComponent(value)}`

  if (options?.path) cookie += `; path=${options.path}`
  if (options?.maxAge) cookie += `; max-age=${options.maxAge}`
  if (options?.expires) cookie += `; expires=${options.expires.toUTCString()}`
  if (options?.sameSite) cookie += `; samesite=${options.sameSite}`
  if (options?.secure) cookie += `; secure`

  document.cookie = cookie
}
