import { createHash } from 'node:crypto';

/**
 * Canonical JSON as project-arbol-sdk-js spells it: object keys sorted by UTF-16 code unit (`sort()`),
 * recursively; arrays in order; no whitespace. Kept apart from json.ts's `canonical`, which sorts with
 * `localeCompare`, so both repositories hash the same document to the same value.
 */
function canonical(value: unknown): string {
  if (Array.isArray(value)) {
    return `[${value.map(canonical).join(',')}]`;
  }
  if (value !== null && typeof value === 'object') {
    const record = value as Record<string, unknown>;
    return `{${Object.keys(record)
      .sort()
      .map((key) => `${JSON.stringify(key)}:${canonical(record[key])}`)
      .join(',')}}`;
  }
  return JSON.stringify(value);
}

/**
 * The OpenAPI's content hash: SHA-256 (hex) of its canonical JSON without `info.version`. A release
 * whose document is otherwise the same has the same hash; Cargo.toml records it beside the release.
 */
export function openapiContentSha256(document: unknown): string {
  const copy = structuredClone(document) as { info?: { version?: unknown } };
  if (copy?.info && typeof copy.info === 'object') {
    delete copy.info.version;
  }
  return createHash('sha256').update(canonical(copy)).digest('hex');
}
