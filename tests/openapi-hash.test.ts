import { describe, expect, test } from 'bun:test';

import { openapiContentSha256 } from '../generation/openapi-hash';

// The definition project-arbol-sdk-js uses, verbatim: both repositories must agree on every document.
const reference = (input: unknown): string => {
  const canonical = (value: unknown): string =>
    Array.isArray(value)
      ? `[${value.map(canonical).join(',')}]`
      : value !== null && typeof value === 'object'
        ? `{${Object.keys(value)
            .sort()
            .map((key) => `${JSON.stringify(key)}:${canonical((value as Record<string, unknown>)[key])}`)
            .join(',')}}`
        : JSON.stringify(value);
  const document = structuredClone(input) as { info: { version?: string } };
  delete document.info.version;
  return new Bun.CryptoHasher('sha256').update(canonical(document)).digest('hex');
};

const document = {
  openapi: '3.1.0',
  info: { title: 'Arbol', version: '1.20261017.200040' },
  paths: { '/v1/b': {}, '/v1/a': { get: { tags: ['Z', 'a'], parameters: [{ name: 'é' }, { name: 'B' }] } } },
  components: { schemas: { b: { type: 'string', enum: ['x', null, 2, true] }, B: {}, _: {}, a: {} } },
};

describe('openapiContentSha256', () => {
  test('matches the definition sdk-js uses', () => {
    expect(openapiContentSha256(document)).toBe(reference(document));
  });

  test('ignores info.version, key order and whitespace', () => {
    const later = JSON.parse(JSON.stringify({ ...document, info: { version: '1.20261018.0', title: 'Arbol' } }, null, 2));
    expect(openapiContentSha256(later)).toBe(openapiContentSha256(document));
  });

  test('changes with the content', () => {
    const changed = structuredClone(document);
    changed.paths['/v1/b'] = { get: {} } as never;
    expect(openapiContentSha256(changed)).not.toBe(openapiContentSha256(document));
  });

  test('leaves the document as it was', () => {
    const copy = structuredClone(document);
    openapiContentSha256(copy);
    expect(copy).toEqual(document);
  });
});
