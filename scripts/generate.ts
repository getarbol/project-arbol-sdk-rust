import { resolve } from 'node:path';

import { generate } from '../generation/generate';

const [openapi, extra] = process.argv.slice(2);
if (!openapi || extra) {
  throw new Error('Usage: bun scripts/generate.ts <openapi.json>');
}
await generate(resolve(openapi), resolve(import.meta.dir, '..'));
