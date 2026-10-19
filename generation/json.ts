import { createHash } from 'node:crypto';

/** JSON helpers the merge and the generators share. */
export type JsonObject = Record<string, unknown>;
export function object(value: unknown): value is JsonObject {
  return value !== null && typeof value === 'object' && !Array.isArray(value);
}
/** Exact object inputs, including unions: keep each branch intact instead of flattening its fields. */
export function objectInput(value: unknown): value is JsonObject {
  if (!object(value)) {
    return false;
  }
  if (value.type === 'object' && object(value.properties)) {
    return true;
  }
  const branches = value.anyOf ?? value.oneOf;
  return (
    value.type === undefined && Array.isArray(branches) && branches.length > 0 && branches.every(objectInput)
  );
}
export function sha256(value: string | Uint8Array): string {
  return createHash('sha256').update(value).digest('hex');
}

/** One spelling per value: keys sorted, undefined left out, so equal documents compare equal. */
export function canonical(value: unknown): string {
  if (Array.isArray(value)) {
    return `[${value.map(canonical).join(',')}]`;
  }
  if (object(value)) {
    return `{${Object.entries(value)
      .filter(([, v]) => v !== undefined)
      .toSorted(([a], [b]) => a.localeCompare(b))
      .map(([k, v]) => `${JSON.stringify(k)}:${canonical(v)}`)
      .join(',')}}`;
  }
  return JSON.stringify(value);
}
