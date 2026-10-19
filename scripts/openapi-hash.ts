import { readFile } from 'node:fs/promises';

import { openapiContentSha256 } from '../generation/openapi-hash';

/** Prints the OpenAPI's content hash (see generation/openapi-hash.ts). */
const [openapi, extra] = process.argv.slice(2);
if (!openapi || extra) {
  throw new Error('Usage: bun scripts/openapi-hash.ts <openapi.json>');
}
console.log(openapiContentSha256(JSON.parse(await readFile(openapi, 'utf-8'))));
