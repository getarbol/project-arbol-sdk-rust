/**
 * The Rust SDK's generated module (`src/generated`), as a Forge plugin: a serde type for
 * every model (`types.rs`), how each published operation is reached (`operations.rs`), and one async
 * method per operation, `arbol.agenda().appointments().list(&input)` (`client.rs`). The transport is
 * hand-written (`src/lib.rs`), with the TypeScript SDK's rules.
 *
 * Types follow the schema: an optional field is `Option<T>` and left out when `None`; an optional field
 * that may also be null is `Option<Option<T>>`, so `Some(None)` sends `null`; a union is an untagged enum
 * whose members' literal fields tell them apart; anything Rust cannot say exactly is `serde_json::Value`.
 */
import { definePlugin } from '@cloudflare/forge/plugin';
import type { Field, Forge, Model, ScalarType, Type } from '@cloudflare/forge/plugin';

import { allModels, file, surfaces } from './shared';
import type { Operation } from './shared';

const header = '// Generated from the pinned Arbol release by Cloudflare Forge. Do not edit.\n';

const KEYWORDS = new Set(
  'as async await break const continue crate dyn else enum extern false fn for gen if impl in let loop match mod move mut pub ref return static struct super trait true type unsafe use where while abstract become box do final macro override priv try typeof unsized virtual yield'.split(
    ' '
  )
);

/** A snake_case identifier for a field or method, escaped when it is a keyword. */
function snakeIdent(words: readonly string[]): string {
  let name = words.join('_') || 'value';
  if (/^\d/.test(name)) {
    name = `_${name}`;
  }
  if (name === 'self' || name === 'super' || name === 'crate' || name === 'Self') {
    return `${name}_`;
  }
  return KEYWORDS.has(name) ? `r#${name}` : name;
}

/** PascalCase from any text: words of letters and digits. */
function pascal(text: string): string {
  const words = text
    .replaceAll(/([a-z0-9])([A-Z])/g, '$1 $2')
    .split(/[^A-Za-z0-9]+/)
    .filter(Boolean);
  const name = words.map((word) => word.charAt(0).toUpperCase() + word.slice(1)).join('') || 'Value';
  return /^\d/.test(name) ? `V${name}` : name;
}

/**
 * A documentation comment. The text comes from a service's OpenAPI: every line is a comment line, and
 * no character that would end a line or the comment (a carriage return, a control character, a Unicode
 * line or paragraph separator) survives.
 */
function docs(text: string, indent: string): string {
  const clean = text
    .replaceAll(/\r\n?/g, '\n')
    // oxlint-disable-next-line no-control-regex -- removing control characters is the point.
    .replaceAll(/[\u0000-\u0008\u000B-\u001F\u007F\u2028\u2029]/g, ' ');
  return clean
    ? `${clean
        .split('\n')
        .map((line) => `${indent}///${line ? ` ${line}` : ''}`)
        .join('\n')}\n`
    : '';
}

/** A Rust string literal for any text: quotes, backslashes and control characters escaped as Rust reads them. */
function rustString(value: string): string {
  let out = '"';
  for (const character of value) {
    const code = character.codePointAt(0) ?? 0;
    if (character === '"' || character === '\\') {
      out += `\\${character}`;
    } else if (code < 0x20 || code === 0x7f || code === 0x20_28 || code === 0x20_29) {
      out += `\\u{${code.toString(16)}}`;
    } else {
      out += character;
    }
  }
  return `${out}"`;
}

const isNull = (type: Type) => type.kind === 'scalar' && type.scalar === 'null';

/** The models reachable from a type without passing through a Vec or a map, which need no Box. */
function direct(type: Type, out: Set<Model>): void {
  if (type.kind === 'model') {
    out.add(type.model);
  } else if (type.kind === 'object') {
    for (const field of type.fields) {
      direct(field.type, out);
    }
  } else if (type.kind === 'union' || type.kind === 'intersection') {
    for (const member of type.members) {
      direct(member, out);
    }
  }
}

const SIGNED = new Set(['int', 'int8', 'int16', 'int32', 'int64']);
const UNSIGNED = new Set(['uint8', 'uint16', 'uint32', 'uint64']);

/** The Rust type of a scalar: integers by their format, strings for every textual form. */
function scalar(type: ScalarType): string {
  if (type.scalar === 'boolean') {
    return 'bool';
  }
  if (type.scalar === 'number') {
    if (SIGNED.has(type.number)) {
      return 'i64';
    }
    return UNSIGNED.has(type.number) ? 'u64' : 'f64';
  }
  if (['string', 'date-time', 'date', 'bytes', 'binary'].includes(type.scalar)) {
    return 'String';
  }
  return 'serde_json::Value';
}

class Emitter {
  readonly names = new Map<Model, string>();
  readonly items: string[] = [];
  readonly taken = new Set<string>();
  /** For each model, every model it reaches directly (transitively). */
  readonly reach = new Map<Model, Set<Model>>();

  constructor(models: readonly Model[]) {
    for (const model of models) {
      this.names.set(model, this.unique(pascal(model.schemaName)));
    }
    const edges = new Map<Model, Set<Model>>();
    for (const model of models) {
      const out = new Set<Model>();
      direct(model.type, out);
      edges.set(model, out);
    }
    for (const model of models) {
      const seen = new Set<Model>();
      const stack = [...(edges.get(model) ?? [])];
      while (stack.length) {
        const next = stack.pop() as Model;
        if (!seen.has(next)) {
          seen.add(next);
          stack.push(...(edges.get(next) ?? []));
        }
      }
      this.reach.set(model, seen);
    }
  }

  unique(name: string): string {
    let candidate = name;
    for (let n = 2; this.taken.has(candidate); n += 1) {
      candidate = `${name}${n}`;
    }
    this.taken.add(candidate);
    return candidate;
  }

  /** The Rust type of `type`, declaring a named item for an inline object, enum or union. */
  rust(type: Type, hint: string, owner: Model | null): string {
    switch (type.kind) {
      case 'scalar': {
        return scalar(type);
      }
      case 'enum': {
        return this.enumeration(type.values, hint);
      }
      case 'array': {
        return `Vec<${this.rust(type.elem, `${hint}Item`, null)}>`;
      }
      case 'object': {
        if (!type.fields.length) {
          return type.extra
            ? `std::collections::BTreeMap<String, ${this.rust(type.extra, `${hint}Value`, null)}>`
            : 'serde_json::Map<String, serde_json::Value>';
        }
        return this.structure(type.fields, type.extra !== undefined, hint, '', owner);
      }
      case 'union': {
        const members = type.members.filter((member) => !isNull(member));
        if (members.length === 1 && members[0]) {
          return `Option<${this.rust(members[0], hint, owner)}>`;
        }
        const inner = this.union(members, hint, owner);
        return members.length < type.members.length ? `Option<${inner}>` : inner;
      }
      case 'intersection': {
        const parts = type.members.map((member, index) => ({
          rust: this.rust(member, `${hint}Part${index + 1}`, owner),
          object: resolvesToObject(member),
        }));
        if (!parts.every((part) => part.object)) {
          return 'serde_json::Value';
        }
        const name = this.unique(hint);
        this.items.push(
          `#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]\npub struct ${name} {\n${parts
            .map((part, index) => `    #[serde(flatten)]\n    pub part${index + 1}: ${part.rust},`)
            .join('\n')}\n}\n`
        );
        return name;
      }
      case 'model': {
        const name = this.names.get(type.model) as string;
        // A model that reaches its own owner directly is boxed, so the type has a size.
        return owner && (type.model === owner || this.reach.get(type.model)?.has(owner))
          ? `Box<${name}>`
          : name;
      }
      default: {
        const never: never = type;
        throw new Error(`Unknown type ${JSON.stringify(never)}`);
      }
    }
  }

  enumeration(values: readonly (string | number | boolean)[], hint: string): string {
    if (!values.every((value) => typeof value === 'string')) {
      return 'serde_json::Value';
    }
    const name = this.unique(hint);
    const used = new Set<string>();
    const variants = (values as string[]).map((value) => {
      let variant = pascal(value);
      for (let n = 2; used.has(variant); n += 1) {
        variant = `${pascal(value)}${n}`;
      }
      used.add(variant);
      return `    #[serde(rename = ${rustString(value)})]\n    ${variant},`;
    });
    this.items.push(
      `#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]\npub enum ${name} {\n${variants.join('\n')}\n}\n`
    );
    return name;
  }

  union(members: readonly Type[], hint: string, owner: Model | null): string {
    const name = this.unique(hint);
    const used = new Set<string>();
    const variants = members.map((member, index) => {
      let variant =
        member.kind === 'model' ? (this.names.get(member.model) as string) : `Variant${index + 1}`;
      const literal = member.kind === 'object' ? discriminant(member.fields) : null;
      if (literal) {
        variant = pascal(literal);
      }
      for (let n = 2; used.has(variant); n += 1) {
        variant = `${variant}${n}`;
      }
      used.add(variant);
      return `    ${variant}(${this.rust(member, `${name}${variant}`, owner)}),`;
    });
    this.items.push(
      `#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]\n#[serde(untagged)]\npub enum ${name} {\n${variants.join('\n')}\n}\n`
    );
    return name;
  }

  structure(
    fields: readonly Field[],
    open: boolean,
    hint: string,
    description: string,
    owner: Model | null,
    fixed?: string
  ): string {
    const name = fixed ?? this.unique(hint);
    const used = new Set<string>();
    const lines = fields.map((field) => {
      let ident = snakeIdent(field.name.words);
      for (let n = 2; used.has(ident); n += 1) {
        ident = `${snakeIdent(field.name.words)}_${n}`;
      }
      used.add(ident);
      const nullable = field.type.kind === 'union' && field.type.members.some(isNull);
      const inner = this.rust(field.type, `${name}${pascal(field.name.words.join(' '))}`, owner);
      const attributes = [`rename = ${rustString(field.wireName)}`];
      let rustType = inner;
      if (!field.required) {
        rustType = `Option<${inner}>`;
        attributes.push('default', 'skip_serializing_if = "Option::is_none"');
        if (nullable) {
          // Absent, null or a value: Some(None) sends null.
          attributes.push('with = "crate::nullable"');
        }
      }
      return `${docs(field.description, '    ')}    #[serde(${attributes.join(', ')})]\n    pub ${ident}: ${rustType},`;
    });
    if (open) {
      lines.push(
        '    /// Every other key the object carries.\n    #[serde(flatten)]\n    pub extra: serde_json::Map<String, serde_json::Value>,'
      );
    }
    const defaultable = fields.every((field) => !field.required);
    this.items.push(
      `${docs(description, '')}#[derive(Debug, Clone, PartialEq, ${defaultable ? 'Default, ' : ''}serde::Serialize, serde::Deserialize)]\npub struct ${name} {\n${lines.join('\n')}\n}\n`
    );
    return name;
  }

  model(model: Model): void {
    const name = this.names.get(model) as string;
    const { type } = model;
    if (type.kind === 'object' && type.fields.length) {
      this.structure(type.fields, type.extra !== undefined, name, model.description, model, name);
      return;
    }
    // An enum or a union without null is declared under the model's own name; anything else is an
    // alias, whose inline items take the name `<Model>Inner`.
    const named = type.kind === 'enum' || (type.kind === 'union' && !type.members.some(isNull));
    if (named) {
      this.taken.delete(name);
    }
    const rust = this.rust(type, named ? name : `${name}Inner`, model);
    if (rust !== name) {
      this.items.push(`${docs(model.description, '')}pub type ${name} = ${rust};\n`);
    }
  }
}

/** A field whose type is a single string literal names the union member it belongs to. */
function discriminant(fields: readonly Field[]): string | null {
  for (const field of fields) {
    if (field.required && field.type.kind === 'enum' && field.type.values.length === 1) {
      const [value] = field.type.values;
      if (typeof value === 'string') {
        return value;
      }
    }
  }
  return null;
}

function resolvesToObject(type: Type): boolean {
  if (type.kind === 'model') {
    return resolvesToObject(type.model.type);
  }
  return type.kind === 'object' && type.fields.length > 0;
}

function types(forge: Forge): { source: string; names: Map<string, string> } {
  const models = allModels(forge);
  const emitter = new Emitter(models);
  for (const model of models) {
    emitter.model(model);
  }
  const names = new Map(models.map((model) => [model.schemaName, emitter.names.get(model) as string]));
  return {
    source: `${header}#![allow(clippy::all, clippy::pedantic, missing_docs)]\n\n${emitter.items.join('\n')}`,
    names,
  };
}

const constName = (id: string) =>
  id
    .replaceAll(/([a-z0-9])([A-Z])/g, '$1_$2')
    .replaceAll(/[^A-Za-z0-9]+/g, '_')
    .toUpperCase();

const KIND: Record<string, string> = {
  string: 'String',
  integer: 'Integer',
  number: 'Number',
  boolean: 'Boolean',
  list: 'List',
};

function operations(rest: readonly Operation[], release: string): string {
  const consts = rest.map((op) => {
    const query = op.query
      ? `Some(&[${Object.entries(op.query)
          .map(([field, kind]) => `(${rustString(field)}, QueryKind::${KIND[kind]})`)
          .join(', ')}])`
      : 'None';
    return `${docs(`\`${op.id}\`: ${op.title}`, '')}pub const ${constName(op.id)}: Operation = Operation {
    name: ${rustString(op.id)},
    method: ${rustString(op.method)},
    path: ${rustString(op.path)},
    path_field: ${op.pathField ? `Some(${rustString(op.pathField)})` : 'None'},
    query: ${query},
    effect: Effect::${op.effect.charAt(0).toUpperCase()}${op.effect.slice(1)},
    max_output_bytes: ${op.maxOutputBytes},
};
`;
  });
  return `${header}use crate::{Effect, Operation, QueryKind};

/// The platform release these operations come from.
pub const RELEASE: &str = ${rustString(release)};

${consts.join('\n')}`;
}

function client(rest: readonly Operation[], names: Map<string, string>): string {
  const byNamespace = new Map<string, Map<string, Operation[]>>();
  for (const op of rest) {
    const resources = byNamespace.get(op.namespace) ?? new Map<string, Operation[]>();
    byNamespace.set(op.namespace, resources);
    const list = resources.get(op.resource ?? '') ?? [];
    resources.set(op.resource ?? '', list);
    list.push(op);
  }
  const method = (op: Operation) => {
    const input = names.get(op.inputType);
    const output = names.get(op.outputType);
    if (!input || !output) {
      throw new Error(`${op.id}: Forge has no model of ${input ? op.outputType : op.inputType}`);
    }
    const verb = snakeIdent(
      op.verb
        .replaceAll(/([a-z0-9])([A-Z])/g, '$1 $2')
        .toLowerCase()
        .split(/[\s-]+/)
    );
    return `${docs([op.title, op.description].filter((t, i, all) => t && all.indexOf(t) === i).join('\n\n'), '    ')}    pub async fn ${verb}(&self, input: &types::${input}) -> Result<types::${output}, Error> {
        self.0.call(&operations::${constName(op.id)}, input).await
    }`;
  };
  const parts: string[] = [];
  const roots: string[] = [];
  for (const [namespace, resources] of byNamespace) {
    const nsType = `${pascal(namespace)}Api`;
    const nsMethods: string[] = [];
    for (const [resource, ops] of resources) {
      if (!resource) {
        nsMethods.push(...ops.map(method));
        continue;
      }
      const resourceType = `${pascal(namespace)}${pascal(resource)}Api`;
      nsMethods.push(
        `    pub fn ${snakeIdent(
          resource
            .replaceAll(/([a-z0-9])([A-Z])/g, '$1 $2')
            .toLowerCase()
            .split(/[\s-]+/)
        )}(&self) -> ${resourceType}<'a> {\n        ${resourceType}(self.0)\n    }`
      );
      parts.push(
        `/// \`${namespace}.${resource}\`\n#[derive(Debug, Clone, Copy)]\npub struct ${resourceType}<'a>(&'a Client);\n\nimpl<'a> ${resourceType}<'a> {\n${ops.map(method).join('\n\n')}\n}\n`
      );
    }
    parts.push(
      `/// \`${namespace}\`\n#[derive(Debug, Clone, Copy)]\npub struct ${nsType}<'a>(&'a Client);\n\nimpl<'a> ${nsType}<'a> {\n${nsMethods.join('\n\n')}\n}\n`
    );
    roots.push(
      `    pub fn ${snakeIdent(namespace.split('-'))}(&self) -> ${nsType}<'_> {\n        ${nsType}(self)\n    }`
    );
  }
  return `${header}#![allow(clippy::all, clippy::pedantic)]

use crate::generated::{operations, types};
use crate::{Client, Error};

impl Client {
${roots.join('\n\n')}
}

${parts.join('\n')}`;
}

export const { generate, generateGroup } = definePlugin({
  generate(forge) {
    const { rest } = surfaces(forge);
    const ops = rest.map(({ op }) => op);
    const { source, names } = types(forge);
    return [
      file('mod.rs', `${header}pub mod client;\npub mod operations;\npub mod types;\n`),
      file('types.rs', source),
      file('operations.rs', operations(ops, forge.version)),
      file('client.rs', client(ops, names)),
    ];
  },
});
