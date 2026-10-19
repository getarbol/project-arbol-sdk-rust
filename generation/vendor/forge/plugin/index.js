/**
 * Write forge plugins in TypeScript or JavaScript.
 *
 * A plugin generates files from the forge of an OpenAPI document: its
 * groups, methods, models and types, as forge's Rust API has them. Build it
 * with `forge-plugin build`, and run it with
 * `forge openapi.json --target wasm://plugin.wasm --out sdk`.
 *
 * ```ts
 * import { definePlugin } from "@cloudflare/forge/plugin";
 *
 * export const { generate, generateGroup } = definePlugin({
 *   generateGroup(group) {
 *     const names = group.methods.map((m) => m.name.camel());
 *     return [{ path: `${group.name.kebab()}.txt`, content: names.join("\n") }];
 *   },
 * });
 * ```
 *
 * The forge stays in forge's process. Each part is fetched the first time
 * a plugin reads it, and kept until the call ends, so reading a part again
 * is free, and the same part is always the same object: compare groups,
 * methods and models with `===`.
 *
 * @module
 */
// This file is the source of plugin/index.js and plugin/index.d.ts, which
// scripts/build.mjs compiles it to. It is typed against the bindings jco
// generates from plugin/forge.wit, so a change to the WIT that this library
// doesn't follow fails to compile. Its exports are interfaces, and the
// classes behind them stay private, so the bindings never show in
// index.d.ts.
import * as host from 'forge:plugin/forge@0.2.0';
/**
 * Returns the exports of a plugin that calls `plugin`. Export them from the
 * plugin's entry module:
 *
 * ```ts
 * export const { generate, generateGroup } = definePlugin({ ... });
 * ```
 *
 * Throw to fail: forge stops and prints the error's message.
 */
export function definePlugin(plugin) {
    const exports = {
        generate() {
            return call(() => files(plugin.generate?.(new ForgeImpl()) ?? []));
        },
        generateGroup(id) {
            return call(() => files(plugin.generateGroup?.(new ForgeImpl().group(id)) ?? []));
        },
    };
    return exports;
}
// The bindings' types say a missing option is undefined, but QuickJS gives
// null, so options are read through `opt`.
/** Reads the JSON text the host sends for a value. */
function parseJson(text) {
    return JSON.parse(text);
}
/** Returns `value`, or undefined for null. */
function opt(value) {
    return value ?? undefined;
}
/**
 * Runs `f`, turning whatever it throws into the string the WIT's error
 * case carries.
 */
function call(f) {
    try {
        return f();
    }
    catch (err) {
        throw err instanceof Error ? err.message : String(err);
    }
}
/** Returns `files` as the host takes them, with their content in bytes. */
function files(files) {
    return Array.from(files, (file) => {
        if (typeof file?.path !== 'string') {
            throw new Error(`a file needs a path, not ${JSON.stringify(file?.path)}`);
        }
        const content = file.content ?? '';
        return { path: file.path, content: typeof content === 'string' ? encode(content) : content };
    });
}
const NativeEncoder = globalThis.TextEncoder;
/**
 * Encodes `s` in UTF-8. QuickJS has no TextEncoder, so `s` is encoded here:
 * a code unit at a time, into one buffer as large as UTF-8 can need, which
 * is several times faster than building an array of bytes. A lone surrogate
 * is written as its own three bytes.
 */
const encode = NativeEncoder
    ? (s) => new NativeEncoder().encode(s)
    : (s) => {
        const out = new Uint8Array(s.length * 3);
        let n = 0;
        for (let i = 0; i < s.length; i++) {
            let c = s.charCodeAt(i);
            if (c < 0x80) {
                out[n++] = c;
            }
            else if (c < 0x800) {
                out[n++] = 0xc0 | (c >> 6);
                out[n++] = 0x80 | (c & 63);
            }
            else {
                const next = c >= 0xd800 && c < 0xdc00 ? s.charCodeAt(i + 1) : 0;
                if (next >= 0xdc00 && next < 0xe000) {
                    c = 0x10000 + ((c - 0xd800) << 10) + (next - 0xdc00);
                    i++;
                    out[n++] = 0xf0 | (c >> 18);
                    out[n++] = 0x80 | ((c >> 12) & 63);
                }
                else {
                    out[n++] = 0xe0 | (c >> 12);
                }
                out[n++] = 0x80 | ((c >> 6) & 63);
                out[n++] = 0x80 | (c & 63);
            }
        }
        return out.slice(0, n);
    };
/** Returns a list of ids, which comes as a Uint32Array, as an array. */
function ids(list) {
    return Array.from(list);
}
/** Returns a function that returns what `f` makes, made once on first use. */
function lazy(f) {
    let done = false;
    let value;
    return () => {
        if (!done) {
            value = f();
            done = true;
        }
        return value;
    };
}
/** Returns what `cache` has under `id`, making it with `make` if it has nothing. */
function cached(cache, id, make) {
    let value = cache.get(id);
    if (value === undefined) {
        value = make();
        cache.set(id, value);
    }
    return value;
}
class NameImpl {
    words;
    constructor(words) {
        this.words = Object.freeze(Array.from(words));
    }
    get isEmpty() {
        return this.words.length === 0;
    }
    camel() {
        return this.words.map((word, i) => (i === 0 ? word : upperFirst(word))).join('');
    }
    pascal() {
        return this.words.map(upperFirst).join('');
    }
    snake() {
        return this.words.join('_');
    }
    kebab() {
        return this.words.join('-');
    }
    equals(other) {
        return this.words.length === other.words.length && this.words.every((word, i) => word === other.words[i]);
    }
    toString() {
        return this.words.join(' ');
    }
}
/** Upper-cases the first character of `word`, as Rust's `char::to_uppercase` does. */
function upperFirst(word) {
    const first = word.codePointAt(0);
    if (first === undefined) {
        return word;
    }
    const ch = String.fromCodePoint(first);
    return ch.toUpperCase() + word.slice(ch.length);
}
class ForgeImpl {
    version;
    description;
    baseUrl;
    groupCache = new Map();
    methodCache = new Map();
    modelCache = new Map();
    typeCache = new Map();
    allGroups = lazy(() => ids(host.groups()).map((id) => this.group(id)));
    topModels = lazy(() => ids(host.models()).map((id) => this.model(id)));
    constructor() {
        const info = host.info();
        this.version = info.version;
        this.description = info.description;
        this.baseUrl = info.baseUrl;
    }
    get groups() {
        return this.allGroups();
    }
    get models() {
        return this.topModels();
    }
    group(id) {
        return cached(this.groupCache, id, () => new GroupImpl(this, id));
    }
    method(id) {
        return cached(this.methodCache, id, () => new MethodImpl(this, id));
    }
    model(id) {
        return cached(this.modelCache, id, () => new ModelImpl(this, id));
    }
    type(id) {
        return cached(this.typeCache, id, () => makeType(this, id));
    }
}
class GroupImpl {
    forge;
    info;
    constructor(forge, id) {
        this.forge = forge;
        this.info = lazy(() => {
            const g = host.getGroup(id);
            return {
                name: new NameImpl(g.name),
                description: g.description,
                path: ids(g.path),
                children: ids(g.children),
                methods: ids(g.methods),
                models: ids(g.models),
            };
        });
    }
    get name() {
        return this.info().name;
    }
    get description() {
        return this.info().description;
    }
    get path() {
        return this.info().path.map((id) => this.forge.group(id).name);
    }
    get parent() {
        const path = this.info().path;
        const parent = path[path.length - 2];
        return parent === undefined ? undefined : this.forge.group(parent);
    }
    get children() {
        return this.info().children.map((id) => this.forge.group(id));
    }
    get descendants() {
        const out = [];
        const stack = [...this.children].reverse();
        for (let group = stack.pop(); group !== undefined; group = stack.pop()) {
            out.push(group);
            stack.push(...[...group.children].reverse());
        }
        return out;
    }
    get methods() {
        return this.info().methods.map((id) => this.forge.method(id));
    }
    get models() {
        return this.info().models.map((id) => this.forge.model(id));
    }
}
/** How each HTTP method is sent, but `other`, which carries its own. */
const httpMethods = {
    get: 'GET',
    put: 'PUT',
    post: 'POST',
    delete: 'DELETE',
    options: 'OPTIONS',
    head: 'HEAD',
    patch: 'PATCH',
    trace: 'TRACE',
    query: 'QUERY',
    connect: 'CONNECT',
};
class MethodImpl {
    forge;
    id;
    info;
    constructor(forge, id) {
        this.forge = forge;
        this.id = id;
        this.info = lazy(() => {
            const m = host.getMethod(id);
            const body = opt(m.body);
            return {
                name: new NameImpl(m.name),
                group: m.group,
                httpMethod: m.httpMethod.tag === 'other' ? m.httpMethod.val : httpMethods[m.httpMethod.tag],
                path: m.path,
                summary: m.summary,
                description: m.description,
                deprecated: m.deprecated,
                availability: opt(m.availability),
                hidden: m.hidden,
                confirmation: opt(m.confirmation),
                request: opt(m.request),
                headers: opt(m.headers),
                body: body && {
                    required: body.required,
                    description: body.description,
                    ...contents(forge, body.contents, body.json),
                },
                responses: m.responses.map((r) => response(forge, r)),
                errors: m.errors.map((r) => response(forge, r)),
                extensions: m.extensions.map(([name, value]) => [name, parseJson(value)]),
            };
        });
    }
    get name() {
        return this.info().name;
    }
    get group() {
        return this.forge.group(this.info().group);
    }
    get httpMethod() {
        return this.info().httpMethod;
    }
    get path() {
        return this.info().path;
    }
    get summary() {
        return this.info().summary;
    }
    get description() {
        return this.info().description;
    }
    get deprecated() {
        return this.info().deprecated;
    }
    get availability() {
        return this.info().availability;
    }
    get hidden() {
        return this.info().hidden;
    }
    get confirmation() {
        return this.info().confirmation;
    }
    get request() {
        const request = this.info().request;
        return request === undefined ? undefined : this.forge.type(request);
    }
    get headers() {
        const headers = this.info().headers;
        if (headers === undefined) {
            return undefined;
        }
        const type = this.forge.type(headers);
        return type.kind === 'object' ? type : undefined;
    }
    get body() {
        return this.info().body;
    }
    get responses() {
        return this.info().responses;
    }
    get errors() {
        return this.info().errors;
    }
    get extensions() {
        return this.info().extensions;
    }
    extension(name) {
        return this.info().extensions.find(([n]) => n === name)?.[1];
    }
    location(field) {
        if (!(field instanceof FieldImpl) || field.forge !== this.forge) {
            return undefined;
        }
        const location = opt(host.getLocation(this.id, field.object, field.index));
        switch (location?.tag) {
            case undefined:
                return undefined;
            case 'path':
                return { in: 'path' };
            case 'query':
                return { in: 'query', exploded: location.val };
            default: {
                const unknown = location;
                throw new Error(`a location this library doesn't know: ${JSON.stringify(unknown)}`);
            }
        }
    }
}
function response(forge, r) {
    return { status: r.status, description: r.description, ...contents(forge, r.contents, r.json) };
}
/** Returns `list` as `Content`s, and the one at index `json`. */
function contents(forge, list, json) {
    const all = list.map((c) => new ContentImpl(forge, c));
    const index = opt(json);
    return { contents: all, json: index === undefined ? undefined : all[index] };
}
class ContentImpl {
    forge;
    mediaType;
    description;
    ty;
    constructor(forge, info) {
        this.forge = forge;
        this.mediaType = info.mediaType;
        this.description = info.description;
        this.ty = info.ty;
    }
    get isJson() {
        const base = (this.mediaType.split(';')[0] ?? '').trim().toLowerCase();
        return base === 'application/json' || base.endsWith('+json');
    }
    get type() {
        return this.forge.type(this.ty);
    }
}
class ModelImpl {
    forge;
    info;
    constructor(forge, id) {
        this.forge = forge;
        this.info = lazy(() => {
            const m = host.getModel(id);
            return {
                schemaName: m.schemaName,
                name: new NameImpl(m.name),
                group: opt(m.group),
                description: m.description,
                deprecated: m.deprecated,
                ty: m.ty,
            };
        });
    }
    get schemaName() {
        return this.info().schemaName;
    }
    get name() {
        return this.info().name;
    }
    get group() {
        const group = this.info().group;
        return group === undefined ? undefined : this.forge.group(group);
    }
    get description() {
        return this.info().description;
    }
    get deprecated() {
        return this.info().deprecated;
    }
    get type() {
        return this.forge.type(this.info().ty);
    }
    toString() {
        return this.schemaName;
    }
}
class FieldImpl {
    forge;
    object;
    index;
    wireName;
    name;
    required;
    description;
    deprecated;
    readOnly;
    sensitive;
    default;
    ty;
    /** The object type the field is in, and its index there, which the host knows it by. */
    constructor(forge, object, index, info) {
        this.forge = forge;
        this.object = object;
        this.index = index;
        this.wireName = info.wireName;
        this.name = new NameImpl(info.name);
        this.required = info.required;
        this.description = info.description;
        this.deprecated = info.deprecated;
        this.readOnly = info.readOnly;
        this.sensitive = info.sensitive;
        const value = opt(info.default);
        this.default = value === undefined ? undefined : parseJson(value);
        this.ty = info.ty;
    }
    get type() {
        return this.forge.type(this.ty);
    }
}
/** Returns the type `id`. Types it refers to are made when first read. */
function makeType(forge, id) {
    const t = host.getType(id);
    const type = (ty) => forge.type(ty);
    switch (t.tag) {
        case 'scalar':
            return t.val.tag === 'number'
                ? { kind: 'scalar', scalar: 'number', number: t.val.val }
                : { kind: 'scalar', scalar: t.val.tag };
        case 'enum':
            return { kind: 'enum', values: t.val.map((value) => value.val) };
        case 'array': {
            const { elem } = t.val;
            const maxItems = opt(t.val.maxItems);
            return {
                kind: 'array',
                get elem() {
                    return type(elem);
                },
                // A u64 comes as a bigint; no array's limit needs more than a number.
                maxItems: maxItems === undefined ? undefined : Number(maxItems),
            };
        }
        case 'object': {
            const fields = t.val.fields.map((field, i) => new FieldImpl(forge, id, i, field));
            const extra = opt(t.val.extra);
            return {
                kind: 'object',
                fields,
                get extra() {
                    return extra === undefined ? undefined : type(extra);
                },
            };
        }
        case 'union': {
            const members = ids(t.val.members);
            const d = opt(t.val.discriminator);
            const discriminator = d && {
                wireName: d.wireName,
                name: new NameImpl(d.name),
                get cases() {
                    return d.cases.map(([value, ty]) => [value, type(ty)]);
                },
                get default() {
                    const model = opt(d.default);
                    return model === undefined ? undefined : forge.model(model);
                },
            };
            return {
                kind: 'union',
                get members() {
                    return members.map(type);
                },
                exclusive: t.val.exclusive,
                discriminator,
            };
        }
        case 'intersection': {
            const members = ids(t.val);
            return {
                kind: 'intersection',
                get members() {
                    return members.map(type);
                },
            };
        }
        case 'model':
            return { kind: 'model', model: forge.model(t.val) };
        default: {
            // A kind the WIT gains fails to compile here until it is handled.
            const unknown = t;
            throw new Error(`type ${id} is of a kind this library doesn't know: ${JSON.stringify(unknown)}`);
        }
    }
}
