import { mkdtemp, readFile, readdir, rm, writeFile, mkdir, cp } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { dirname, join, relative, resolve } from 'node:path';

import { buildPlugin, forgeBinary, forgeLock, runPlugin } from '../scripts/forge';
import { setOpenapi } from './cargo';
import { canonical, object, sha256 } from './json';
import { openapiContentSha256 } from './openapi-hash';
import type { JsonObject } from './json';
import { preparePublicOpenApi } from './public-openapi';
import type { Operation } from './public-openapi';

/**
 * The crate's generated module comes from Cloudflare Forge: one document, the generation document, and
 * the WebAssembly plugin generation/plugins/sdk-rust.ts. Forge models the document's groups, methods and
 * types; the plugin writes src/generated/*.rs from that model and refuses a method its operation does
 * not match.
 */
const TARGETS = [{ plugin: 'sdk-rust', out: 'src/generated', files: /^[a-z_]+\.rs$/ }] as const;

/** A reference to one of the document's component schemas. */
function schemaReference(name: string) {
  return { $ref: `#/components/schemas/${name}` };
}

/**
 * The release's human permissions, as its OpenAPI's person flow lists them. The generation document
 * carries them for every Arbol plugin (the CLI requests them at login); a release without them is
 * refused, as it is in the other clients.
 */
export function humanScopes(components: JsonObject): string[] {
  const schemes = (components.securitySchemes ?? {}) as JsonObject;
  const human = (schemes.arbolHuman ?? {}) as {
    flows?: { authorizationCode?: { scopes?: Record<string, string> } };
  };
  const scopes = Object.keys(human.flows?.authorizationCode?.scopes ?? {}).toSorted();
  if (!scopes.length) {
    throw new Error('The contract names no person permission (arbolHuman authorizationCode scopes).');
  }
  return scopes;
}

/** The document's schemas and the operations' outputs; one name never means two schemas. */
function mergeSchemas(parts: readonly JsonObject[]): JsonObject {
  const merged: JsonObject = {};
  for (const part of parts) {
    for (const [name, schema] of Object.entries(part)) {
      if (name in merged && canonical(merged[name]) !== canonical(schema)) {
        throw new Error(`Schema ${name} has two definitions`);
      }
      merged[name] = schema;
    }
  }
  return merged;
}

/**
 * A schema as Forge types it. Forge 0.1 reads no `patternProperties`, so an object keyed by pattern
 * (`{ "^[a-z]+$": … }`) would type as an empty object. With `additionalProperties: false` its values
 * become `additionalProperties`, which is what TypeScript and Rust can say; without it, other keys may
 * hold anything, so the object stays open. A type is never stricter than its owner's schema (an MCP
 * output schema checks results against it). Only typing sees this: the CLI validates against the exact
 * schemas.
 */
export function typingSchema(schema: unknown): unknown {
  if (Array.isArray(schema)) {
    return schema.map(typingSchema);
  }
  if (!object(schema)) {
    return schema;
  }
  const next: JsonObject = Object.fromEntries(
    Object.entries(schema).map(([key, value]) => [key, typingSchema(value)])
  );
  if (object(next.patternProperties)) {
    const values = Object.values(next.patternProperties);
    if (next.additionalProperties === false) {
      // Only keys matching a pattern: every value is one of the patterns' values.
      next.additionalProperties = values.length === 1 ? values[0] : { anyOf: values };
    } else if (next.additionalProperties !== undefined) {
      next.additionalProperties = { anyOf: [next.additionalProperties, ...values] };
    }
    // Otherwise any other key may hold any value: the object stays open, never stricter than its owner's.
    delete next.patternProperties;
  }
  return next;
}

/**
 * The one document Forge models: the public OpenAPI's operations (`x-arbol-surface: rest`), every schema
 * a client types, and the hidden carrier of what belongs to the whole document (`document`).
 */
export function generationDocument(
  document: JsonObject,
  operations: readonly Operation[],
  schemas: JsonObject,
  personScopes: readonly string[]
): JsonObject {
  const byId = new Map(operations.map((op) => [op.id, op]));
  const paths: JsonObject = {};
  for (const [path, item] of Object.entries(document.paths as JsonObject)) {
    const next: JsonObject = {};
    for (const [method, raw] of Object.entries(item as JsonObject)) {
      if (!object(raw) || typeof raw.operationId !== 'string') {
        next[method] = raw;
        continue;
      }
      const op = byId.get(raw.operationId);
      if (!op) {
        throw new Error(`${raw.operationId} is not a prepared operation`);
      }
      next[method] = { ...raw, 'x-arbol-surface': 'rest', 'x-arbol-operation': op };
    }
    paths[path] = next;
  }
  paths['/_arbol/document'] = {
    get: {
      operationId: 'arbol.document',
      summary: 'The generation document',
      'x-fern-sdk-group-name': 'arbol-generation',
      'x-fern-sdk-method-name': 'document',
      'x-forge-hidden': true,
      'x-arbol-surface': 'document',
      'x-arbol-document': {
        personScopes,
        schemas: ((document.components as JsonObject).schemas ?? {}) as JsonObject,
      },
      responses: {
        // Every schema a client types is a model: Forge models what the document's methods use.
        200: {
          description: 'Every schema.',
          content: {
            'application/json': {
              schema: {
                type: 'object',
                properties: Object.fromEntries(
                  Object.keys(schemas).map((name) => [name, schemaReference(name)])
                ),
              },
            },
          },
        },
      },
    },
  };
  return {
    ...document,
    paths,
    components: { ...(document.components as JsonObject), schemas: typingSchema(schemas) },
  };
}

/** Every file under a directory, by its path relative to it. */
async function filesUnder(directory: string): Promise<string[]> {
  const entries = await readdir(directory, { recursive: true, withFileTypes: true });
  return entries
    .filter((entry) => entry.isFile())
    .map((entry) => relative(directory, join(entry.parentPath, entry.name)))
    .toSorted();
}

/**
 * A plugin's source: every file of generation/plugins, by path and bytes. The compiled component is not
 * reproducible from one build to the next, so it cannot name the plugin; with Forge's commit, this does.
 */
async function pluginSource(repository: string): Promise<string> {
  const directory = resolve(repository, 'generation/plugins');
  const files = await filesUnder(directory);
  const parts = await Promise.all(
    files.map(async (path) => `${path}\0${sha256(await readFile(join(directory, path)))}`)
  );
  return sha256(parts.join('\n'));
}

/**
 * src/generated/ from the Arbol API's public OpenAPI, with its provenance in src/generated/generation.json
 * and the OpenAPI's release and content hash in Cargo.toml's [package.metadata.arbol.openapi].
 */
export async function generate(openapiPath: string, repository: string): Promise<void> {
  const bytes = await readFile(openapiPath, 'utf-8');
  const { document, operations, outputs, version } = preparePublicOpenApi(JSON.parse(bytes));
  const lock = await forgeLock();
  const components = document.components as JsonObject;
  const schemas = mergeSchemas([components.schemas as JsonObject, outputs]);
  const generation = generationDocument(document, operations, schemas, humanScopes(components));
  const binary = await forgeBinary(lock);
  const work = await mkdtemp(join(tmpdir(), 'arbol-generation-'));
  try {
    const documentPath = join(work, 'generation.json');
    await writeFile(documentPath, JSON.stringify(generation));
    const written: { path: string; sha256: string }[] = [];
    const plugins: Record<string, string> = {};
    for (const target of TARGETS) {
      const plugin = await buildPlugin(target.plugin);
      plugins[target.plugin] = await pluginSource(repository);
      const out = join(work, target.plugin);
      runPlugin(binary, plugin, documentPath, out);
      const files = await filesUnder(out);
      if (!files.length || files.some((path) => !target.files.test(path))) {
        throw new Error(`${target.plugin} wrote unexpected files: ${files.join(', ')}`);
      }
      const destination = resolve(repository, target.out);
      await rm(destination, { recursive: true, force: true });
      await mkdir(dirname(destination), { recursive: true });
      await cp(out, destination, { recursive: true });
      for (const path of files) {
        written.push({ path: `${target.out}/${path}`, sha256: sha256(await readFile(join(out, path))) });
      }
    }
    const contentSha256 = openapiContentSha256(JSON.parse(bytes));
    // Provenance: which release, which Forge and which plugins wrote which bytes.
    await writeFile(
      resolve(repository, 'src/generated/generation.json'),
      `${JSON.stringify(
        {
          formatVersion: 6,
          release: version,
          openapiSha256: sha256(bytes),
          // What decides a release: the document's content without info.version (Generate compares it).
          openapiContentSha256: contentSha256,
          forge: lock.upstream,
          plugins,
          outputs: written.filter((output) => !output.path.endsWith('generation.json')),
        },
        null,
        2
      )}\n`
    );
    const manifest = resolve(repository, 'Cargo.toml');
    await writeFile(manifest, setOpenapi(await readFile(manifest, 'utf-8'), { release: version, sha256: contentSha256 }));
  } finally {
    await rm(work, { recursive: true, force: true });
  }
}
