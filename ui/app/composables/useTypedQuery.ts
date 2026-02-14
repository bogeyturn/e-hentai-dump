import {useRoute} from '#imports'

type Parser<T> = (value: unknown) => T | undefined

export const string = (): Parser<string> =>
    (v) => {
        if (typeof v === 'string') {
            return v
        } else {
            throw new Error('Not a string')
        }
    }

export const number = (): Parser<number> =>
    (v) => {
        if (typeof v !== 'string') throw new Error('Not a string')
        const n = Number(v)
        if (Number.isInteger(n) && n >= 0) {
            return n
        }
        throw new Error('Not a positive integer')
    }
export const boolean = (): Parser<boolean> =>
    (v) => {
        if (typeof v !== "string") throw new Error("Not a string");

        const s = v.trim().toLowerCase();

        if (s === "true" || s === "on") return true;
        if (s === "false" || s === "off") return false;

        throw new Error('Not a boolean (expected true/false/on/off)');
    };

export const enumOf = <T extends readonly string[]>(values: T): Parser<T[number]> =>
    (v) => {
        if (typeof v === 'string' && values.includes(v as T[number])) {
            return (v as T[number])
        } else {
            throw new Error('Not a valid enum value')
        }
    }

type Schema<T> = {
    [K in keyof T]: Parser<T[K]>
}
export const withDefaults = <T>(parser: Parser<T>, defaultValue: T): Parser<T> =>
    (v) => {
        if (v == null || v === '') {
            return defaultValue
        }
        return parser(v)
    }

export const optional = <T>(parser: Parser<T>): Parser<T> =>
    (v) => {
        if (v == null || v === '') {
            return undefined
        }
        return parser(v)
    }

export function useTypedQuery<T>(schema: Schema<T>) {
    const route = useRoute()

    return computed(() => {
        const result: Partial<T> = {}
        const errors: string[] = []

        for (const key in schema) {
            try {
                result[key as keyof T] = schema[key](route.query[key])
            } catch {
                errors.push(`Invalid query parameter: ${key}`)
            }
        }

        return errors.length
            ? {error: errors.join(', ')}
            : {data: result as T}
    })
}