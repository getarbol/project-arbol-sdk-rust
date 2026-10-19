import { kebab, queryKind } from './projection';
import type { QueryKind } from './projection';

import { reachesItself } from './references';
import { object, objectInput } from './json';
import type { JsonObject } from './json';

export type { JsonObject };
export type { QueryKind };

// oxlint-disable-next-line no-control-regex -- generated code embeds these as comments: no control character.
const TITLE = /^[^\u0000-\u001F\u007F\u2028\u2029]{1,200}$/;
// oxlint-disable-next-line no-control-regex -- line breaks only as LF.
const DESCRIPTION = /^[^\u0000-\u0009\u000B-\u001F\u007F\u2028\u2029]{0,8000}$/;

/** Text every client embeds: refused, not escaped, when it could end the comment that holds it. */
function words(value: string, rule: RegExp, at: string): string {
  if (!rule.test(value)) {
    throw new Error(`${at} has a control character or is too long`);
  }
  return value;
}
export interface InputField {
  name: string;
  flag: string;
  kind: QueryKind;
  required: boolean;
  description: string;
}
export interface Operation {
  /** The public name the service declares: `agenda.appointments.list`. */
  id: string;
  namespace: string;
  resource: string | null;
  verb: string;
  method: 'GET' | 'POST' | 'PATCH' | 'DELETE';
  path: string;
  /** The input field carried in the path. */
  pathField: string | null;
  /** GET: every other input field and its query representation. Other methods send one JSON body. */
  query: Record<string, QueryKind> | null;
  command: string[];
  /** Top-level scalar and list fields the CLI accepts as flags. */
  fields: InputField[];
  inputSchema: JsonObject;
  inputType: string;
  inputOptional: boolean;
  /** The named type of the data a successful call returns (the response envelope's `data`). */
  outputType: string;
  title: string;
  description: string;
  effect: 'read' | 'write' | 'delete';
  scopes: string[];
  scopeMatch: 'any' | 'all';
  actorKinds: ('user' | 'service')[];
  maxOutputBytes: number;
}
const METHODS = ['get', 'post', 'patch', 'delete'] as const;
const SEGMENT = /^[a-z][a-zA-Z0-9]*$/;
const SERVICE = /^[a-z][a-z0-9-]*$/;
const KINDS = new Set(['string', 'integer', 'number', 'boolean', 'list']);
const pascal = (value: string) =>
  value
    .split(/[-.]/)
    .map((part) => part.charAt(0).toUpperCase() + part.slice(1))
    .join('');

function assertLocalReferences(value: unknown): void {
  if (Array.isArray(value)) {
    for (const item of value) {
      assertLocalReferences(item);
    }
    return;
  }
  if (!object(value)) {
    return;
  }
  if ('$ref' in value && (typeof value.$ref !== 'string' || !value.$ref.startsWith('#/components/'))) {
    throw new Error('Contracts must be bundled: only local #/components references are permitted.');
  }
  // Remote schema resolution is never a generation-time network dependency.
  if ('$dynamicRef' in value || '$recursiveRef' in value) {
    throw new Error('Dynamic schema references require an explicit compiler adapter.');
  }
  for (const child of Object.values(value)) {
    assertLocalReferences(child);
  }
}

/** A body field the CLI can take as a flag: a scalar or a list of strings, else none (Maximus' rule). */
function scalarKind(
  schema: unknown,
  components: JsonObject,
  recursive: (name: string) => boolean
): QueryKind | null {
  return queryKind(schema, (reference) => {
    const name = /^#\/components\/schemas\/(.+)$/.exec(reference)?.[1];
    // A schema that reaches itself is never a scalar, and would never end.
    return name && !recursive(name) ? components[name] : undefined;
  });
}

/**
 * The `data` of a success envelope (`{ success, data, error, metadata }`), through a component
 * reference or a union of envelopes, never the branch whose `success` is `false`.
 */
function successData(
  schema: unknown,
  components: JsonObject,
  seen = new Set<unknown>()
): JsonObject | undefined {
  if (!object(schema) || seen.has(schema)) {
    return undefined;
  }
  seen.add(schema);
  const name =
    typeof schema.$ref === 'string' ? /^#\/components\/schemas\/(.+)$/.exec(schema.$ref)?.[1] : undefined;
  if (name) {
    return successData(components[name], components, seen);
  }
  const properties = object(schema.properties) ? schema.properties : undefined;
  if (
    properties &&
    object(properties.data) &&
    !(object(properties.success) && properties.success.const === false)
  ) {
    return properties.data;
  }
  for (const branch of [
    ...(Array.isArray(schema.oneOf) ? schema.oneOf : []),
    ...(Array.isArray(schema.anyOf) ? schema.anyOf : []),
  ]) {
    const data = successData(branch, components, seen);
    if (data) {
      return data;
    }
  }
  return undefined;
}

/**
 * The public OpenAPI as the generators read it: every operation's prepared description (public name,
 * route, input fields, effect, scopes, limits), checked so a private or incomplete operation never
 * becomes a client method.
 */
export function preparePublicOpenApi(source: unknown): {
  document: JsonObject;
  operations: Operation[];
  /** Each operation's output schema by its type name, for the SDK's types only (not the CLI's). */
  outputs: JsonObject;
  version: string;
} {
  if (
    !object(source) ||
    source.openapi !== '3.1.0' ||
    !object(source.paths) ||
    !object(source.info) ||
    typeof source.info.version !== 'string' ||
    !/^[a-zA-Z0-9][a-zA-Z0-9._-]{0,127}$/.test(source.info.version)
  ) {
    throw new Error('A bundled OpenAPI 3.1.0 document with a release version is required.');
  }
  assertLocalReferences(source);
  const document = structuredClone(source);
  if (!object(document.components)) {
    document.components = {};
  }
  const components = document.components as JsonObject;
  if (!object(components.schemas)) {
    components.schemas = {};
  }
  const schemas = components.schemas as JsonObject;
  const recursive = reachesItself(schemas);
  const operations: Operation[] = [];
  const outputs: JsonObject = {};
  const ids = new Set<string>();
  const commands = new Set<string>();
  for (const [path, item] of Object.entries(document.paths as JsonObject)) {
    if (!object(item) || '$ref' in item || !/^\/v1\/[a-z0-9-]+(?:\/[A-Za-z0-9_{}-]+)*$/.test(path)) {
      throw new Error(`Invalid public path: ${path}`);
    }
    for (const [method, raw] of Object.entries(item)) {
      if (
        method === 'parameters' ||
        method === 'summary' ||
        method === 'description' ||
        method.startsWith('x-')
      ) {
        continue;
      }
      if (!(METHODS as readonly string[]).includes(method) || !object(raw)) {
        throw new Error(`Unsupported method ${method} at ${path}`);
      }
      const id = raw.operationId;
      const api = raw['x-arbol-api'];
      const publication = raw['x-arbol-publish'];
      if (
        !object(api) ||
        typeof api.namespace !== 'string' ||
        !SERVICE.test(api.namespace) ||
        api.namespace === 'cnn' ||
        typeof api.verb !== 'string' ||
        !SEGMENT.test(api.verb) ||
        typeof api.resource !== 'string' ||
        !SEGMENT.test(api.resource) ||
        id !== [api.namespace, api.resource, api.verb].filter(Boolean).join('.') ||
        raw['x-arbol-service-id'] !== api.namespace ||
        path.split('/')[2] !== api.namespace ||
        raw['x-arbol-publication'] !== 'selected' ||
        !object(publication) ||
        publication.rest !== true ||
        typeof publication.mcp !== 'boolean'
      ) {
        throw new Error(
          `Operation ${String(id)} has no valid public name and REST publication (CNN is always private).`
        );
      }
      if (ids.has(id)) {
        throw new Error(`Duplicate operation id ${id}`);
      }
      ids.add(id);
      const input = raw['x-arbol-input-schema'];
      const scopes = raw['x-required-scopes'];
      const scopeMatch = raw['x-scope-match'];
      const effect = raw['x-effect'];
      const actorKinds = raw['x-arbol-actor-kinds'];
      if (!objectInput(input)) {
        throw new Error(`${id}: missing exact owner object input schema`);
      }
      if (
        !Array.isArray(scopes) ||
        !scopes.length ||
        !scopes.every((s) => typeof s === 'string' && s.length > 0) ||
        (scopeMatch !== 'any' && scopeMatch !== 'all') ||
        (effect !== 'read' && effect !== 'write' && effect !== 'delete')
      ) {
        throw new Error(`${id}: missing finite permission/effect metadata`);
      }
      if (
        !Array.isArray(actorKinds) ||
        !actorKinds.length ||
        !actorKinds.every((kind) => kind === 'user' || kind === 'service')
      ) {
        throw new Error(`${id}: missing explicit actor eligibility`);
      }
      if (method === 'get' && effect !== 'read') {
        throw new Error(`${id}: GET cannot publish a mutation`);
      }
      if (!object(raw.responses) || !Object.keys(raw.responses).some((s) => /^2\d\d$/.test(s))) {
        throw new Error(`${id}: missing response contract`);
      }
      const pathField = typeof api.id === 'string' ? api.id : null;
      if (!object(input.properties) && (pathField || method === 'get')) {
        throw new Error(`${id}: path and query inputs must be one object`);
      }
      const placeholders = [...path.matchAll(/\{([^}]+)\}/g)].map((match) => match[1]);
      if (JSON.stringify(placeholders) !== JSON.stringify(pathField ? [pathField] : [])) {
        throw new Error(`${id}: path parameter differs from its api id`);
      }
      const query = api.query === null || api.query === undefined ? null : api.query;
      if (
        (method === 'get') !== (query !== null) ||
        (query !== null && (!object(query) || !Object.values(query).every((kind) => KINDS.has(String(kind)))))
      ) {
        throw new Error(`${id}: a GET needs a typed query map; body methods have none`);
      }
      // Forge models an operation only through these names: its command and groups, then its method.
      raw['x-fern-sdk-group-name'] = [api.namespace, ...(api.resource ? [api.resource] : [])].join('.');
      raw['x-fern-sdk-method-name'] = api.verb;
      const command = [api.namespace, ...(api.resource ? [kebab(api.resource)] : []), kebab(api.verb)];
      if (commands.has(command.join(' '))) {
        throw new Error(`${id}: command collision`);
      }
      commands.add(command.join(' '));
      const required = Array.isArray(input.required) ? (input.required as string[]) : [];
      // A union travels as one JSON body. Branch-specific scalar flags would lose its constraints.
      const properties = object(input.properties) ? input.properties : {};
      const fields: InputField[] = Object.entries(
        query ??
          Object.fromEntries(
            Object.keys(properties).flatMap((field) => {
              const kind = scalarKind(properties[field], schemas, recursive);
              return kind ? [[field, kind]] : [];
            })
          )
      )
        .filter(([field]) => field !== pathField)
        .map(([field, kind]) => ({
          name: field,
          flag: kebab(field),
          kind: kind as QueryKind,
          required: required.includes(field),
          description:
            object(properties[field]) && typeof (properties[field] as JsonObject).description === 'string'
              ? String((properties[field] as JsonObject).description)
              : '',
        }));
      const inputType = `${pascal(id)}Input`;
      if (inputType in schemas) {
        throw new Error(`${id}: generated input schema name collides with owner schema`);
      }
      schemas[inputType] = input;
      // The data of the success envelope ({ success, data, error, metadata }) is what a client returns.
      const success = Object.entries(raw.responses as JsonObject).find(([status]) =>
        /^2\d\d$/.test(status)
      )?.[1];
      const content =
        object(success) && object(success.content) ? success.content['application/json'] : undefined;
      const data = successData(object(content) ? content.schema : undefined, schemas);
      if (!data) {
        throw new Error(`${id}: the success response names no data schema`);
      }
      const outputType = `${pascal(id)}Output`;
      if (outputType in schemas || outputType in outputs) {
        throw new Error(`${id}: generated output schema name collides with another schema`);
      }
      outputs[outputType] = data;
      const outputLimit = raw['x-max-output-bytes'] ?? 16 * 1024 * 1024;
      if (
        typeof outputLimit !== 'number' ||
        !Number.isSafeInteger(outputLimit) ||
        outputLimit <= 0 ||
        outputLimit > 16 * 1024 * 1024
      ) {
        throw new Error(`${id}: invalid output byte limit`);
      }
      operations.push({
        id,
        namespace: api.namespace,
        resource: api.resource as string | null,
        verb: api.verb,
        method: method.toUpperCase() as Operation['method'],
        path,
        pathField,
        query: query as Record<string, QueryKind> | null,
        command,
        fields,
        inputSchema: input,
        inputType,
        inputOptional: object(input.properties) && required.length === 0,
        outputType,
        title: words(typeof raw.summary === 'string' ? raw.summary : id, TITLE, `${id}: summary`),
        description: words(
          typeof raw.description === 'string' ? raw.description : '',
          DESCRIPTION,
          `${id}: description`
        ),
        effect,
        scopes,
        scopeMatch,
        actorKinds,
        maxOutputBytes: outputLimit,
      });
    }
  }
  if (!operations.length) {
    throw new Error('The public OpenAPI has no operation.');
  }
  // Structural checks above establish the boundary; no schema dialect conversion is performed.
  return {
    document,
    operations: operations.toSorted((a, b) => a.id.localeCompare(b.id)),
    outputs,
    version: source.info.version,
  };
}
