import { afterEach, describe, expect, test } from 'bun:test';
import { cp, mkdir, mkdtemp, readFile, rm, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';

import { buildPlugin, forgeBinary, forgeLock, runPlugin } from '../scripts/forge';
import { openapi } from '../generation/cargo';
import { generate, generationDocument, typingSchema } from '../generation/generate';
import { openapiContentSha256 } from '../generation/openapi-hash';
import { preparePublicOpenApi } from '../generation/public-openapi';
import { fixture } from './fixture';

const root = resolve(import.meta.dir, '..');
const temporary: string[] = [];
afterEach(async () => {
  for (const dir of temporary.splice(0)) {
    await rm(dir, { recursive: true, force: true });
  }
});

/** A scratch repository: what generation reads besides the OpenAPI (the plugins and Cargo.toml). */
async function repository(): Promise<string> {
  const directory = await mkdtemp(join(tmpdir(), 'arbol-sdk-rust-'));
  temporary.push(directory);
  await mkdir(join(directory, 'generation'), { recursive: true });
  await cp(join(root, 'generation/plugins'), join(directory, 'generation/plugins'), { recursive: true });
  await cp(join(root, 'Cargo.toml'), join(directory, 'Cargo.toml'));
  return directory;
}

describe('public OpenAPI boundary', () => {
  test('preserves literal, nullable, discriminated input and permission alternatives', () => {
    const value = preparePublicOpenApi(fixture());
    expect(value.document.openapi).toBe('3.1.0');
    expect(value.operations[0]?.scopeMatch).toBe('any');
    expect(value.operations[0]?.inputSchema).toEqual(
      fixture().paths['/v1/agenda/appointments'].post['x-arbol-input-schema']
    );
  });
  test.each([
    'cnn',
    'private',
    'absent',
    'mcp-only',
    'unversioned',
    'remote-ref',
    'missing-actors',
    'unnamed',
    'renamed',
  ])('%s fails closed', (kind) => {
    const value = fixture();
    const op = value.paths['/v1/agenda/appointments'].post;
    if (kind === 'unnamed') {
      delete (op as Partial<typeof op>)['x-arbol-api'];
    }
    if (kind === 'renamed') {
      op.operationId = 'agenda.createAppointment';
    }
    if (kind === 'cnn') {
      op['x-arbol-service-id'] = 'cnn';
    }
    if (kind === 'private') {
      op['x-arbol-publication'] = 'private';
    }
    if (kind === 'absent') {
      delete (op as Partial<typeof op>)['x-arbol-publication'];
    }
    if (kind === 'mcp-only') {
      op['x-arbol-publish'].rest = false;
    }
    if (kind === 'remote-ref') {
      Object.assign(op['x-arbol-input-schema'], { $ref: 'https://example.invalid/private.json' });
    }
    if (kind === 'missing-actors') {
      delete (op as Partial<typeof op>)['x-arbol-actor-kinds'];
    }
    if (kind === 'unversioned') {
      (value.info as { version?: string }).version = '';
    }
    expect(() => preparePublicOpenApi(value)).toThrow();
  });
});

describe('Forge generation', () => {
  test('the real plugin writes the crate with native types, deterministically, and records the OpenAPI', async () => {
    const directory = await repository();
    const document = fixture();
    await writeFile(join(directory, 'openapi.json'), JSON.stringify(document));
    await generate(join(directory, 'openapi.json'), directory);
    const client = await readFile(join(directory, 'src/generated/client.rs'), 'utf-8');
    expect(client).toContain(
      'pub async fn create(&self, input: &types::AgendaAppointmentsCreateInput) -> Result<types::AgendaAppointmentsCreateOutput, Error>'
    );
    const types = await readFile(join(directory, 'src/generated/types.rs'), 'utf-8');
    // The union's members are told apart by their literal kind; a nullable id stays nullable.
    expect(types).toContain('#[serde(untagged)]');
    expect(types).toContain('pub id: Option<String>,');
    const operations = await readFile(join(directory, 'src/generated/operations.rs'), 'utf-8');
    expect(operations).toContain('pub const RELEASE: &str = "0.0.0";');
    const first = await readFile(join(directory, 'src/generated/generation.json'), 'utf-8');
    const provenance = JSON.parse(first);
    expect(provenance.forge.commit).toBe('7f86447210b7bea3779bfdab3cd5655eade0eadb');
    expect(provenance.release).toBe('0.0.0');
    expect(provenance.openapiContentSha256).toBe(openapiContentSha256(document));
    expect(provenance.outputs.map((output: { path: string }) => output.path)).toEqual([
      'src/generated/client.rs',
      'src/generated/mod.rs',
      'src/generated/operations.rs',
      'src/generated/types.rs',
    ]);
    expect(openapi(await readFile(join(directory, 'Cargo.toml'), 'utf-8'))).toEqual({
      release: '0.0.0',
      sha256: openapiContentSha256(document),
    });
    await generate(join(directory, 'openapi.json'), directory);
    expect(await readFile(join(directory, 'src/generated/generation.json'), 'utf-8')).toBe(first);
    // The first run may build Forge with cargo and compiles the plugin.
  }, 900_000);

  test("a service's words stay inside the comments that hold them", async () => {
    const directory = await repository();
    const value = fixture();
    // A description may break lines; what it says must never become code.
    value.paths['/v1/agenda/appointments'].post.description =
      'Books it. */ export const injected = 1; /*\npub const INJECTED: u8 = 1;\n#[test] fn run() {}';
    await writeFile(join(directory, 'openapi.json'), JSON.stringify(value));
    await generate(join(directory, 'openapi.json'), directory);
    const rust = await Promise.all(
      ['client.rs', 'operations.rs', 'types.rs'].map((name) =>
        readFile(join(directory, 'src/generated', name), 'utf-8')
      )
    );
    for (const source of rust) {
      expect(source.split('\n').some((line) => /^\s*(pub const INJECTED|#\[test\])/.test(line))).toBe(false);
    }
    expect(rust.join('\n')).toContain('/// pub const INJECTED: u8 = 1;');
    // A title is one line: a break in it is refused before anything is generated.
    value.paths['/v1/agenda/appointments'].post.summary = 'Book\npub const INJECTED: u8 = 1;';
    await writeFile(join(directory, 'openapi.json'), JSON.stringify(value));
    await expect(generate(join(directory, 'openapi.json'), directory)).rejects.toThrow('summary');
  }, 900_000);

  test('a method Forge models under another name than its operation fails generation', async () => {
    const directory = await repository();
    const { document, operations } = preparePublicOpenApi(fixture());
    const [operation] = operations;
    const { schemas } = document.components as { schemas: Record<string, unknown> };
    // The operation says `book`; Forge models the method from the document's own names, `create`.
    const renamed = generationDocument(document, [{ ...operation!, verb: 'book' }], schemas, ['agenda:read']);
    await writeFile(join(directory, 'generation.json'), JSON.stringify(renamed));
    const binary = await forgeBinary(await forgeLock());
    const plugin = await buildPlugin('sdk-rust');
    expect(() => runPlugin(binary, plugin, join(directory, 'generation.json'), join(directory, 'out'))).toThrow();
  }, 900_000);

  test('Forge types an object keyed by pattern by its values, never stricter than its owner', () => {
    expect(
      typingSchema({
        type: 'object',
        additionalProperties: false,
        patternProperties: { '^[a-z]+$': { type: 'string' } },
      })
    ).toEqual({ type: 'object', additionalProperties: { type: 'string' } });
    // Without additionalProperties, another key may hold any value: the type stays open.
    expect(typingSchema({ type: 'object', patternProperties: { '^[a-z]+$': { type: 'string' } } })).toEqual({
      type: 'object',
    });
  });
});
