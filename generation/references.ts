/** Which components a schema reaches, through `#/components/schemas/<name>` references. */
type Schema = Record<string, unknown>;

const COMPONENT = /^#\/components\/schemas\/(.+)$/;

const isSchema = (value: unknown): value is Schema =>
  value !== null && typeof value === 'object' && !Array.isArray(value);

/** The components a value references, anywhere in it. */
function references(value: unknown, found = new Set<string>()): Set<string> {
  if (Array.isArray(value)) {
    for (const item of value) {
      references(item, found);
    }
  } else if (isSchema(value)) {
    for (const [key, child] of Object.entries(value)) {
      const name = key === '$ref' && typeof child === 'string' ? COMPONENT.exec(child)?.[1] : undefined;
      if (name !== undefined) {
        found.add(name);
      }
      references(child, found);
    }
  }
  return found;
}

/**
 * Whether a component reaches itself through the components it references: such a schema has no end
 * when written out, and is never a scalar a flag or a query string carries.
 */
export function reachesItself(components: Schema): (name: string) => boolean {
  const known = new Map<string, boolean>();
  return (name) => {
    let found = known.get(name);
    if (found === undefined) {
      const seen = new Set<string>();
      const pending = [...references(components[name])];
      found = false;
      for (let next = pending.pop(); next !== undefined && !found; next = pending.pop()) {
        found = next === name;
        if (!seen.has(next)) {
          seen.add(next);
          pending.push(...references(components[next]));
        }
      }
      known.set(name, found);
    }
    return found;
  };
}
