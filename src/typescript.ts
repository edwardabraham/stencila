/**
 * Generate Typescript language bindings.
 */

import fs from 'fs-extra'
import path from 'path'
import { props, read, Schema, types, unions } from './bindings'

/**
 * Generate `dist/types.d.ts` from schemas.
 */
export const build = async (): Promise<string> => {
  const schemas = await read()

  const typesCode = types(schemas)
    .map(typeGenerator)
    .join('')
  const unionsCode = unions(schemas)
    .map(unionGenerator)
    .join('')

  const code = `
${typesCode}

${unionsCode}
`
  const dist = path.join(__dirname, '..', 'dist')
  await fs.ensureDir(dist)

  const file = path.join(dist, 'types.ts')
  await fs.writeFile(file, code)

  // Create an index.js for require.resolve('@stencila/schema') to work
  // properly in Encoda
  // TODO This won't be necessary when using tsc to compile an index.js
  await fs.writeFile(path.join(dist, 'index.js'), '\n')

  return file
}

/**
 * Run `build()` when this file is run as a Node script
 */
// eslint-disable-next-line @typescript-eslint/no-floating-promises
if (module.parent === null) build()

/**
 * Generate a `interface` and a factory function for each type.
 */
export const typeGenerator = (schema: Schema): string => {
  const { title = 'Undefined', extends: parent, description } = schema
  const { own, required, optional } = props(schema)

  const type =
    schema.properties !== undefined
      ? schema.properties.type !== undefined
        ? schema.properties.type.enum !== undefined
          ? schema.properties.type.enum.map(type => `'${type}'`).join(' | ')
          : ''
        : ''
      : ''

  let code = ''

  // Interface
  code += docComment(description)
  code += `export interface ${title} ${
    parent !== undefined ? `extends ${parent}` : ''
  } {\n`
  code += `  type: ${type}\n`
  code += own
    .map(
      ({ name, schema, optional }) =>
        `  ${name}${optional ? `?` : ''}: ${schemaToType(schema)}`
    )
    .join('\n')
  code += '\n}\n\n'

  // Factory function
  code += docComment(`Create a \`${title}\` node`, [
    ...required.map(
      ({ name, schema }) =>
        `@param ${name} {${schemaToType(schema)}} ${schema.description}`
    ),
    `@param options Optional properties`,
    `@returns {${title}}`
  ])
  code += `export const ${funcName(title)} = (\n`
  code += required
    .map(({ name, schema }) => `  ${name}: ${schemaToType(schema)},\n`)
    .join('')
  code += `  options: {\n`
  code += optional
    .map(({ name, schema }) => `    ${name}?: ${schemaToType(schema)}`)
    .join('\n')
  code += `\n  } = {}\n`
  code += `): ${title} => ({\n`
  code += required.map(({ name }) => `  ${name},\n`).join('')
  code += `  ...options,\n`
  code += `  type: '${title}'\n`
  code += '})\n\n'

  return code
}

/**
 * Generate a `Union` type.
 */
export const unionGenerator = (schema: Schema): string => {
  const { title, description } = schema
  let code = docComment(description)
  code += `export type ${title} = ${schemaToType(schema)}\n\n`
  return code
}

/**
 * Generate factory function name
 */
const funcName = (name: string): string => {
  const func = `${name.substring(0, 1).toLowerCase() + name.substring(1)}`
  const reserved: { [key: string]: string } = { delete: 'del' }
  if (reserved[func] !== undefined) return reserved[func]
  else return func
}

/**
 * Generate a JSDoc style comment
 */
const docComment = (description?: string, tags: string[] = []): string => {
  description = description !== undefined ? description : ''
  return (
    '/**\n' +
    ' * ' +
    description.trim().replace('\n', '\n * ') +
    '\n' +
    tags.map(tag => ' * ' + tag.trim().replace('\n', ' ') + '\n').join('') +
    ' */\n'
  )
}

/**
 * Convert a schema definition to a Python type
 */
const schemaToType = (schema: Schema): string => {
  const { type, anyOf, allOf, $ref } = schema

  if ($ref !== undefined) return `${$ref.replace('.schema.json', '')}`
  if (anyOf !== undefined) return anyOfToType(anyOf)
  if (allOf !== undefined) return allOfToType(allOf)
  if (schema.enum !== undefined) return enumToType(schema.enum)

  if (type === 'null') return 'null'
  if (type === 'boolean') return 'boolean'
  if (type === 'number') return 'number'
  if (type === 'integer') return 'number'
  if (type === 'string') return 'string'
  if (type === 'array') return arrayToType(schema)
  if (type === 'object') return '{[key: string]: any}'

  throw new Error(`Unhandled schema: ${JSON.stringify(schema)}`)
}

/**
 * Convert a schema with the `anyOf` property to a Python `Union` type.
 */
const anyOfToType = (anyOf: Schema[]): string => {
  const types = anyOf
    .map(schema => schemaToType(schema))
    .reduce(
      (prev: string[], curr) => (prev.includes(curr) ? prev : [...prev, curr]),
      []
    )
  if (types.length === 0) return ''
  if (types.length === 1) return types[0]
  return types.join(' | ')
}

/**
 * Convert a schema with the `allOf` property to a Python type.
 *
 * If the `allOf` is singular then just use that (this usually arises
 * because the `allOf` is used for a property with a `$ref`). Otherwise,
 * use the last schema (this is usually because one or more codecs can be
 * used on a property and the last schema is the final, expected, type of
 * the property).
 */
const allOfToType = (allOf: Schema[]): string => {
  if (allOf.length === 1) return schemaToType(allOf[0])
  else return schemaToType(allOf[allOf.length - 1])
}

/**
 * Convert a schema with the `array` property to a Typescript `Array` type.
 *
 * Uses the more explicity `Array<>` syntax over the shorter`[]` syntax
 * because the latter necessitates the use of, sometime superfluous, parentheses.
 */
const arrayToType = (schema: Schema): string => {
  const items = Array.isArray(schema.items)
    ? anyOfToType(schema.items)
    : schema.items !== undefined
    ? schemaToType(schema.items)
    : 'any'
  return `Array<${items}>`
}

/**
 * Convert a schema with the `enum` property to Typescript "or values".
 */
export const enumToType = (enu: (string | number)[]): string => {
  return enu
    .map(schema => {
      return JSON.stringify(schema)
    })
    .join(' | ')
}
