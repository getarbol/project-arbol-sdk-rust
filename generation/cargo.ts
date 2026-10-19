/**
 * The value of Cargo.toml that generation writes: the OpenAPI the crate was generated from
 * (`[package.metadata.arbol.openapi]`). Each value is one line, edited in place, so the rest of the file
 * stays as written; a file without that line is refused, never guessed at.
 */

export interface Openapi {
  release: string;
  sha256: string;
}

/** The lines of one TOML table, `[name]`, up to the next table. */
function table(toml: string, name: string): { start: number; end: number } {
  const lines = toml.split('\n');
  const start = lines.findIndex((line) => line.trim() === `[${name}]`);
  if (start === -1) {
    throw new Error(`Cargo.toml has no [${name}]`);
  }
  let end = start + 1;
  while (end < lines.length && !/^\s*\[/.test(lines[end] ?? '')) {
    end += 1;
  }
  return { start, end };
}

/** A string value of a table: `key = "value"`. */
function read(toml: string, name: string, key: string): string {
  const { start, end } = table(toml, name);
  const lines = toml.split('\n').slice(start + 1, end);
  for (const line of lines) {
    const match = new RegExp(`^${key}\\s*=\\s*"([^"\\\\]*)"\\s*$`).exec(line);
    if (match) {
      return match[1] as string;
    }
  }
  throw new Error(`Cargo.toml's [${name}] has no ${key}`);
}

/** `toml` with a string value of a table replaced. */
function write(toml: string, name: string, key: string, value: string): string {
  if (/["\\\n]/.test(value)) {
    throw new Error(`${key} cannot hold ${JSON.stringify(value)}`);
  }
  read(toml, name, key);
  const { start, end } = table(toml, name);
  const lines = toml.split('\n');
  for (let index = start + 1; index < end; index += 1) {
    if (new RegExp(`^${key}\\s*=`).test(lines[index] ?? '')) {
      lines[index] = `${key} = "${value}"`;
    }
  }
  return lines.join('\n');
}

export function openapi(toml: string): Openapi {
  return {
    release: read(toml, 'package.metadata.arbol.openapi', 'release'),
    sha256: read(toml, 'package.metadata.arbol.openapi', 'sha256'),
  };
}

export function setOpenapi(toml: string, value: Openapi): string {
  if (!/^[0-9a-f]{64}$/.test(value.sha256)) {
    throw new Error(`${value.sha256} is not a SHA-256`);
  }
  const named = write(toml, 'package.metadata.arbol.openapi', 'release', value.release);
  return write(named, 'package.metadata.arbol.openapi', 'sha256', value.sha256);
}
