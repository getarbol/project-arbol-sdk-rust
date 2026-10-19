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
/** A plugin: what {@link definePlugin} takes. */
export interface Plugin {
    /** Generates files for the forge. Called exactly once. */
    generate?(forge: Forge): Iterable<File>;
    /**
     * Generates files for a group. Called exactly once for each group, in
     * parallel with other groups, each in an instance of its own. Nothing a
     * call keeps in module state is seen by the calls for other groups.
     */
    generateGroup?(group: Group): Iterable<File>;
}
/**
 * A file a plugin generates.
 *
 * forge writes it to the output directory, creating directories it needs,
 * and skips a file with no content. It rejects a path that is absolute,
 * has `.` or `..` parts, or that two files share.
 */
export interface File {
    /** The path relative to the output directory, using `/` as a separator. */
    path: string;
    /** The content: text, which is written as UTF-8, or bytes. */
    content: string | Uint8Array;
}
/** What the entry module of a plugin exports, under these names. */
export interface Exports {
    generate(): Array<{
        path: string;
        content: Uint8Array;
    }>;
    generateGroup(group: number): Array<{
        path: string;
        content: Uint8Array;
    }>;
}
/**
 * A name from the document, as lower-case words, which a plugin spells in
 * the casing of its language. `XMLDocument` is "xml document", `getByID` is
 * "get by id", and `ipv4_cidr` is "ipv4 cidr".
 */
export interface Name {
    /** The lower-case words. */
    readonly words: readonly string[];
    /** Whether the name has no words: what was written has no letters or digits. */
    readonly isEmpty: boolean;
    /** `petStore` */
    camel(): string;
    /** `PetStore` */
    pascal(): string;
    /** `pet_store` */
    snake(): string;
    /** `pet-store` */
    kebab(): string;
    /** Whether the names have the same words, however they were written. */
    equals(other: Name): boolean;
    /** The words, separated by spaces. */
    toString(): string;
}
/** The OpenAPI document, as an SDK sees it. */
export interface Forge {
    /** `info.version`. */
    readonly version: string;
    /** `info.title`, `info.summary` and `info.description`, as paragraphs. */
    readonly description: string;
    /** The URL of the first server, with its variables substituted. */
    readonly baseUrl: string;
    /** Every group, depth first. Siblings are sorted by name. */
    readonly groups: readonly Group[];
    /** The models in no group: those methods in more than one top-level group use. */
    readonly models: readonly Model[];
}
/** A named collection of subgroups, methods and models. */
export interface Group {
    readonly name: Name;
    /** A short description, or an empty string. */
    readonly description: string;
    /** The names of the groups from the top level down to this one, ending with its own. */
    readonly path: readonly Name[];
    /** The group this one is in, or undefined at the top level. */
    readonly parent: Group | undefined;
    /** The groups directly inside, sorted by name. */
    readonly children: readonly Group[];
    /** Every group below this one, depth first. */
    readonly descendants: readonly Group[];
    /** The methods directly inside, sorted by name. */
    readonly methods: readonly Method[];
    /** The models placed directly in this group: the deepest one every method using them shares. */
    readonly models: readonly Model[];
}
/** An operation an SDK exposes as a function. */
export interface Method {
    /** Unique within its group. */
    readonly name: Name;
    readonly group: Group;
    /** As it is sent, such as `GET`. */
    readonly httpMethod: string;
    /** The path template, such as `/accounts/{account_id}`. */
    readonly path: string;
    readonly summary: string;
    readonly description: string;
    /**
     * Whether the operation is `deprecated`. It is apart from `availability`:
     * a method can be deprecated without being `"deprecated"`, and the other
     * way around.
     */
    readonly deprecated: boolean;
    /** The stage of the API the method is in, from `x-fern-availability`, or undefined if it doesn't say. */
    readonly availability: Availability | undefined;
    /**
     * Whether to leave the method out of listings of methods, such as a CLI's
     * help, although it can still be called: `x-forge-hidden`.
     */
    readonly hidden: boolean;
    /**
     * A sentence to show before calling the method, which changes or deletes
     * something irreversibly, from `x-forge-require-confirmation`, or
     * undefined if it needs no confirmation. An empty string needs
     * confirmation, with no sentence of its own.
     */
    readonly confirmation: string | undefined;
    /**
     * What a caller passes, or undefined for no parameters and no body: the
     * body alone, an object of the path and query parameters, or an
     * intersection of the parameters and the body. The `location` of a field
     * says which parameter it is.
     */
    readonly request: Type | undefined;
    /**
     * The header parameters, as an object with a field for each, sent as the
     * header its wire name names, or undefined if there are none. They are
     * apart from `request`, and their fields have no `location`.
     */
    readonly headers: ObjectType | undefined;
    readonly body: Body | undefined;
    /** The `2XX` responses, sorted by status. */
    readonly responses: readonly Response[];
    /** Every other response, sorted by status with `default` last. */
    readonly errors: readonly Response[];
    /**
     * The extensions of the operation, or of the alias the method comes
     * from, that forge doesn't read itself, in the order they are written:
     * keys starting with `x-`, for plugins to read as they need.
     */
    readonly extensions: readonly (readonly [string, Json])[];
    /** The value of the extension `name`, such as `x-forge-params`, or undefined if there is none. */
    extension(name: string): Json | undefined;
    /**
     * Where `field` is sent, or undefined if it is not one of the method's
     * parameters, as for a field sent in the body.
     */
    location(field: Field): Location | undefined;
}
/**
 * Where a parameter is sent. For the query, `exploded` says whether each
 * item of an array is its own parameter, as in `ids=1&ids=2`.
 */
export type Location = {
    readonly in: 'path';
} | {
    readonly in: 'query';
    readonly exploded: boolean;
};
/** The stage of the API a method is in, as `x-fern-availability` writes it. */
export type Availability = 'alpha' | 'beta' | 'preview' | 'generally-available' | 'deprecated';
/**
 * A JSON value that forge keeps without reading, as `JSON.parse` reads it.
 * Numbers are 64-bit floats, as in forge. A JavaScript object lists keys
 * that are integers first, so an object's keys may not be in the order
 * they are written.
 */
export type Json = null | boolean | number | string | readonly Json[] | {
    readonly [key: string]: Json;
};
export interface Body {
    readonly required: boolean;
    readonly description: string;
    /** Each media type it can be sent as, sorted. */
    readonly contents: readonly Content[];
    /** The JSON content a client sends: `application/json`, or the first JSON media type. */
    readonly json: Content | undefined;
}
export interface Response {
    /** A code such as `200`, a range such as `4XX`, or `default`. */
    readonly status: string;
    readonly description: string;
    readonly contents: readonly Content[];
    readonly json: Content | undefined;
}
/** A body in one media type. */
export interface Content {
    /** Such as `application/json`. */
    readonly mediaType: string;
    /** Whether the media type is `application/json` or ends in `+json`. */
    readonly isJson: boolean;
    /**
     * The description of the content's schema, as a field's is read, or an
     * empty string. The body's or response's own describes it as a whole.
     */
    readonly description: string;
    /** The type of the body; `unknown` for media types other than JSON. */
    readonly type: Type;
}
/** A named schema from `components.schemas`, which an SDK declares as a type. */
export interface Model {
    /** Its key in `components.schemas`, such as `Account`, or a path such as `Tree/$defs/Node`. */
    readonly schemaName: string;
    readonly name: Name;
    /** The group it is placed in, or undefined at the top level. */
    readonly group: Group | undefined;
    readonly description: string;
    readonly deprecated: boolean;
    readonly type: Type;
    /** The schema name. */
    toString(): string;
}
/** A property of an object type. */
export interface Field {
    /** The property's name as it is sent. */
    readonly wireName: string;
    readonly name: Name;
    readonly required: boolean;
    readonly description: string;
    readonly deprecated: boolean;
    /**
     * Whether the server sets the property, so a client doesn't send it,
     * though the same model's field appears in responses: `readOnly`.
     */
    readonly readOnly: boolean;
    /** Whether the value is a secret, such as a password or a token, to keep out of logs: `x-sensitive`. */
    readonly sensitive: boolean;
    /** The `default` of the property's schema, or undefined if it has none. */
    readonly default: Json | undefined;
    readonly type: Type;
}
/**
 * The type of a value. Switch on `kind` to narrow it:
 *
 * ```ts
 * function ts(type: Type): string {
 *   switch (type.kind) {
 *     case "array": return `${ts(type.elem)}[]`;
 *     // ...
 *   }
 * }
 * ```
 *
 * A type that allows null is a union with a `null` scalar member.
 */
export type Type = ScalarType | EnumType | ArrayType | ObjectType | UnionType | IntersectionType | ModelType;
/** A primitive, or any value (`unknown`). */
export type ScalarType = {
    readonly kind: 'scalar';
    readonly scalar: 'unknown' | 'null' | 'boolean' | 'string' | 'date-time' | 'date' | 'bytes' | 'binary';
} | {
    readonly kind: 'scalar';
    readonly scalar: 'number';
    readonly number: NumberFormat;
};
/** The size of a number, from its `format`. `int` and `double` have none. */
export type NumberFormat = 'int' | 'int8' | 'int16' | 'int32' | 'int64' | 'uint8' | 'uint16' | 'uint32' | 'uint64' | 'float' | 'double';
/** One of a list of values: from `enum` or `const`. */
export interface EnumType {
    readonly kind: 'enum';
    /** At least one, all different, in order; never null. */
    readonly values: readonly (string | number | boolean)[];
}
export interface ArrayType {
    readonly kind: 'array';
    readonly elem: Type;
    /** The most elements the array can have, `maxItems`, or undefined if it has no limit. */
    readonly maxItems: number | undefined;
}
/** An object: fields, other keys, both or neither. A map has only `extra`. */
export interface ObjectType {
    readonly kind: 'object';
    readonly fields: readonly Field[];
    /** The type of values under other keys, or undefined if it allows none. */
    readonly extra: Type | undefined;
}
/** Any of the members: from `anyOf`, `oneOf`, or a list of types. */
export interface UnionType {
    readonly kind: 'union';
    /** At least two, all different. */
    readonly members: readonly Type[];
    /**
     * Whether a value is at most one of the members: true for `oneOf`, a
     * list of types, or null and another type; false for `anyOf`.
     */
    readonly exclusive: boolean;
    readonly discriminator: Discriminator | undefined;
}
/** The property whose value says which member of a union a value is. */
export interface Discriminator {
    readonly wireName: string;
    readonly name: Name;
    /** Each value of the property, and the member it says a value is. */
    readonly cases: readonly (readonly [string, Type])[];
    /** The model of a value that matches no case. */
    readonly default: Model | undefined;
}
/** All of the members: from `allOf`. */
export interface IntersectionType {
    readonly kind: 'intersection';
    readonly members: readonly Type[];
}
/** A `$ref` to a model. Read `model.type` to write it in place. */
export interface ModelType {
    readonly kind: 'model';
    readonly model: Model;
}
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
export declare function definePlugin(plugin: Plugin): Exports;
