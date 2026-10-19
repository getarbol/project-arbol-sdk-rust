import { describe, expect, test } from 'bun:test';
import { readFile } from 'node:fs/promises';
import { resolve } from 'node:path';

import { openapi, setOpenapi } from '../generation/cargo';

const manifest = await readFile(resolve(import.meta.dir, '../Cargo.toml'), 'utf-8');
const sha256 = 'a'.repeat(64);

describe('Cargo.toml', () => {
  test('records the OpenAPI, and nothing else changes', () => {
    const next = setOpenapi(manifest, { release: '1.20261020.100', sha256 });
    expect(openapi(next)).toEqual({ release: '1.20261020.100', sha256 });
    // Only the two lines differ.
    const changed = next.split('\n').filter((line, index) => line !== manifest.split('\n')[index]);
    expect(changed).toEqual(['release = "1.20261020.100"', `sha256 = "${sha256}"`]);
  });

  test('refuses what is not a hash, or a value that would leave its line', () => {
    expect(() => setOpenapi(manifest, { release: '1', sha256: 'abc' })).toThrow();
    expect(() => setOpenapi(manifest, { release: '1"\n[x]', sha256 })).toThrow();
    expect(() => openapi('[package]\nname = "arbol-sdk"\n')).toThrow('no [package.metadata.arbol.openapi]');
  });

  test('records the OpenAPI src/generated/ was generated from', () => {
    expect(openapi(manifest).sha256).toMatch(/^[0-9a-f]{64}$/);
    expect(openapi(manifest).release).toMatch(/^\d+\.\d+\.\d+$/);
  });
});
