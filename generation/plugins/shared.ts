/**
 * What every Arbol plugin reads from Forge's model of the generation document (`generation/generate.ts`).
 *
 * Each published operation carries its prepared Arbol operation in `x-arbol-operation` and the surface it
 * belongs to in `x-arbol-surface` (`rest`, the HTTPS API). Forge decides the groups and method
 * names; a plugin refuses a method whose group path, name, HTTP method or path differ from its operation,
 * so a model that drops or renames an operation fails generation instead of passing unnoticed.
 *
 * A plugin reads nothing but the document, so the data of the whole document (the person permissions and
 * the component schemas the CLI validates against) rides on one hidden method, `x-arbol-surface: document`.
 */
import type { Forge, Group, Json, Method, Model } from '@cloudflare/forge/plugin';

export const banner = '// Generated from the pinned Arbol release by Cloudflare Forge. Do not edit.\n';

export type QueryKind = 'string' | 'integer' | 'number' | 'boolean' | 'list';

export interface InputField {
  name: string;
  flag: string;
  kind: QueryKind;
  required: boolean;
  description: string;
}

/** A published operation, as `generation/contract.ts` prepares it. */
export interface Operation {
  id: string;
  namespace: string;
  resource: string | null;
  verb: string;
  method: 'GET' | 'POST' | 'PATCH' | 'DELETE';
  path: string;
  pathField: string | null;
  query: Record<string, QueryKind> | null;
  command: string[];
  fields: InputField[];
  inputSchema: Record<string, unknown>;
  inputType: string;
  inputOptional: boolean;
  outputType: string;
  title: string;
  description: string;
  effect: 'read' | 'write' | 'delete';
  scopes: string[];
  scopeMatch: 'any' | 'all';
  actorKinds: ('user' | 'service')[];
  maxOutputBytes: number;
}

export interface DocumentData {
  personScopes: string[];
  schemas: Record<string, unknown>;
}

export interface Surfaces {
  /** Published operations, by public name, sorted, each with its Forge method. */
  rest: { op: Operation; method: Method }[];
  document: DocumentData;
}

function methods(group: Group): Method[] {
  return [...group.methods, ...group.descendants.flatMap((child) => child.methods)];
}

function extension<T>(method: Method, name: string): T {
  const value = method.extension(name);
  if (value === undefined) {
    throw new Error(`${method.httpMethod} ${method.path} has no ${name}`);
  }
  return value as unknown as T;
}

/** Forge's names, which must be the operation's own: group path, then the method. */
function check(method: Method, expected: string[], what: string): void {
  const actual = [...method.group.path, method.name].map((name) => name.camel());
  const wanted = expected.map((part) => kebabToCamel(part));
  if (actual.join('.') !== wanted.join('.')) {
    throw new Error(`Forge models ${what} as ${actual.join('.')}, not ${wanted.join('.')}`);
  }
}

function kebabToCamel(value: string): string {
  return value.replaceAll(/-([a-z0-9])/g, (_all, c: string) => c.toUpperCase());
}

const byId = <T extends { op: { id: string } }>(a: T, b: T) => a.op.id.localeCompare(b.op.id);

/** Every operation of the document by surface, each checked against Forge's model of it. */
export function surfaces(forge: Forge): Surfaces {
  const all = forge.groups.filter((group) => !group.parent).flatMap(methods);
  const rest: Surfaces['rest'] = [];
  let document: DocumentData | undefined;
  for (const method of all) {
    const surface = extension<string>(method, 'x-arbol-surface');
    if (surface === 'document') {
      document = extension<DocumentData>(method, 'x-arbol-document');
      continue;
    }
    if (surface === 'rest') {
      const op = extension<Operation>(method, 'x-arbol-operation');
      check(method, [op.namespace, ...(op.resource ? [op.resource] : []), op.verb], op.id);
      if (method.httpMethod !== op.method || method.path !== op.path) {
        throw new Error(
          `Forge sends ${op.id} as ${method.httpMethod} ${method.path}, not ${op.method} ${op.path}`
        );
      }
      rest.push({ op, method });
      continue;
    }
    throw new Error(`Unknown surface ${surface}`);
  }
  if (!document) {
    throw new Error('The generation document carries no document data');
  }
  if (new Set(rest.map(({ op }) => op.id)).size !== rest.length) {
    throw new Error('Two methods carry one operation');
  }
  return { rest: rest.toSorted(byId), document };
}

export function json(value: Json | unknown): string {
  return JSON.stringify(value, null, 2);
}

/** A file whose content is text. */
export function file(path: string, content: string) {
  return { path, content };
}

/** Every model of the document, each once, in schema-name order. */
export function allModels(forge: Forge): Model[] {
  const models = new Set<Model>(forge.models);
  // forge.groups lists every group, depth first.
  for (const group of forge.groups) {
    for (const model of group.models) {
      models.add(model);
    }
  }
  return [...models].toSorted((a, b) => codepoints(a.schemaName, b.schemaName));
}

/** Code-point order, the same on every machine (unlike a locale's). */
function codepoints(a: string, b: string): number {
  if (a === b) {
    return 0;
  }
  return a < b ? -1 : 1;
}
