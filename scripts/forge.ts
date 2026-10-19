import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { existsSync } from 'node:fs';
import { mkdir, readFile } from 'node:fs/promises';
import { join, resolve } from 'node:path';

/**
 * Cloudflare Forge, pinned in generation/forge-lock.json: its plugin library is vendored (built from
 * source, every file hashed), and its binary is built here with cargo from the same commit, `--locked`,
 * into .forge/<commit>/ (ignored), then checked against its recorded version. No prebuilt binary is
 * downloaded. Plugins are compiled from generation/plugins with jco into .forge/plugins/.
 *
 * Usage: bun scripts/forge.ts            build what is missing and print the binary's path
 */
const root = resolve(import.meta.dir, '..');

export interface ForgeLock {
  formatVersion: 2;
  upstream: { repository: string; commit: string; license: string; version: string };
  binary: { features: string[] };
  plugin: { jco: string };
  files: Record<string, string>;
}

export async function forgeLock(): Promise<ForgeLock> {
  const lock = JSON.parse(await readFile(join(root, 'generation/forge-lock.json'), 'utf-8')) as ForgeLock;
  if (lock.formatVersion !== 2 || !/^[a-f0-9]{40}$/.test(lock.upstream.commit)) {
    throw new Error('generation/forge-lock.json is not a Forge 0.1 lock');
  }
  for (const [path, hash] of Object.entries(lock.files)) {
    const bytes = await readFile(join(root, 'generation/vendor/forge', path));
    if (createHash('sha256').update(bytes).digest('hex') !== hash) {
      throw new Error(`Unrecorded Forge vendor change: ${path}`);
    }
  }
  return lock;
}

/** The forge binary of the pinned commit, built once with cargo. */
export async function forgeBinary(lock: ForgeLock): Promise<string> {
  const install = join(root, '.forge', lock.upstream.commit);
  const binary = join(install, 'bin', process.platform === 'win32' ? 'forge.exe' : 'forge');
  if (!existsSync(binary)) {
    process.stderr.write(`Building Cloudflare Forge ${lock.upstream.commit.slice(0, 12)} with cargo…\n`);
    execFileSync(
      'cargo',
      [
        'install',
        '--git',
        lock.upstream.repository,
        '--rev',
        lock.upstream.commit,
        '--locked',
        '--features',
        lock.binary.features.join(','),
        '--root',
        install,
        'forge',
      ],
      { stdio: ['ignore', 'inherit', 'inherit'] }
    );
  }
  const version = execFileSync(binary, ['--version'], { encoding: 'utf-8' }).trim();
  if (version !== `forge ${lock.upstream.version}`) {
    throw new Error(`${binary} is ${version}, not forge ${lock.upstream.version}`);
  }
  return binary;
}

const built = new Map<string, string>();

/** A plugin of generation/plugins compiled to a WebAssembly component, once per process. */
export async function buildPlugin(name: string): Promise<string> {
  const known = built.get(name);
  if (known) {
    return known;
  }
  const entry = join(root, 'generation/plugins', `${name}.ts`);
  const out = join(root, '.forge/plugins', `${name}.wasm`);
  await mkdir(join(root, '.forge/plugins'), { recursive: true });
  execFileSync(
    'node',
    [join(root, 'generation/vendor/forge/bin/forge-plugin.mjs'), 'build', entry, '-o', out],
    {
      cwd: root,
      stdio: ['ignore', 'pipe', 'inherit'],
    }
  );
  built.set(name, out);
  return out;
}

/** Run one plugin over a document into a directory, which Forge fills. */
export function runPlugin(binary: string, plugin: string, document: string, out: string): void {
  execFileSync(binary, [document, '--target', `wasm://${plugin}`, '--out', out], {
    stdio: ['ignore', 'pipe', 'inherit'],
  });
}

if (import.meta.main) {
  console.log(await forgeBinary(await forgeLock()));
}
