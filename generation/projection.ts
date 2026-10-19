/**
 * How a schema travels as a command-line flag or in a query string, the same rule the Arbol services
 * and the API apply, so the CLI parses a flag the way the service reads the field.
 */
export type QueryKind = 'string' | 'integer' | 'number' | 'boolean' | 'list';
/** Resolves a local `$ref` (`#/components/schemas/<name>`) of a bundled document. */
export type SchemaResolver = (reference: string) => unknown;

function record(value: unknown): value is Record<string, unknown> {
  return Boolean(value) && typeof value === 'object' && !Array.isArray(value);
}

export function kebab(value: string): string {
  return value.replaceAll(/[A-Z]/g, (letter) => `-${letter.toLowerCase()}`);
}

function resolved(schema: unknown, resolve: SchemaResolver | undefined): unknown {
  let value = schema;
  for (let depth = 0; record(value) && typeof value.$ref === 'string'; depth += 1) {
    if (!resolve || depth > 16) {
      return undefined;
    }
    value = resolve(value.$ref);
  }
  return value;
}

/**
 * How one input field travels in a query string, or null when it cannot: objects and nested lists
 * do not. Lists are comma separated; a `null` alternative is not expressible and is ignored.
 */
export function queryKind(schema: unknown, resolve?: SchemaResolver): QueryKind | null {
  const value = resolved(schema, resolve);
  if (!record(value)) {
    return null;
  }
  let branches: unknown[] | null = null;
  if (Array.isArray(value.anyOf)) {
    branches = value.anyOf;
  } else if (Array.isArray(value.oneOf)) {
    branches = value.oneOf;
  }
  if (branches) {
    const kinds = branches
      .filter((branch) => !(record(branch) && branch.type === 'null'))
      .map((branch) => queryKind(branch, resolve));
    if (!kinds.length || kinds.includes(null)) {
      return null;
    }
    if (kinds.includes('list')) {
      return kinds.every((kind) => kind === 'list' || kind === 'string') ? 'list' : null;
    }
    return kinds.every((kind) => kind === kinds[0]) ? (kinds[0] ?? null) : null;
  }
  if (value.type === 'array') {
    return queryKind(value.items, resolve) === 'string' ? 'list' : null;
  }
  if (
    value.type === 'string' ||
    typeof value.const === 'string' ||
    (Array.isArray(value.enum) && value.enum.every((entry) => typeof entry === 'string'))
  ) {
    return 'string';
  }
  return value.type === 'integer' || value.type === 'number' || value.type === 'boolean' ? value.type : null;
}
