/**
 * A script to convert JSON Schemas from various Rust modules into TypeScript definitions.
 *
 * To regenerate `src/types.ts` run `npm run build` (this ensures
 * that the `index.node` bindings are up to date first).
 * This is done on CI for each release and `src/types.ts` committed.
 *
 * This file is not in `src` because it is not appropriate for it to end up in `dist`
 * since it relies on `devDependencies`.
 */

import fs from 'fs'
import { JSONSchema4 } from 'json-schema'
import { compile } from 'json-schema-to-typescript'
import path from 'path'

const addon = require('./index.node')

;(async () => {
  const schemas = [
    'formatsSchemas',
    'documentsSchemas',
    'patchesSchemas',
    'projectsSchemas',
    'graphsSchemas',
    'sessionsSchemas',
    'sourcesSchemas',
    'pluginsSchema',
    'configSchemas',
    'errorsSchema',
  ].reduce((prev: JSONSchema4[], name: string) => {
    const json = addon[name]()
    const schema = JSON.parse(json)
    return Array.isArray(schema) ? [...prev, ...schema] : [...prev, schema]
  }, [])

  let ts = (
    await Promise.all(
      schemas.map(async (schema) => {
        if (schema.title === 'Error') {
          // Add the error message that gets dynamically
          // added to the error object when it is translated to JSON
          schema.anyOf = (schema.anyOf ?? []).map((subschema): JSONSchema4 => {
            return {
              ...subschema,
              properties: {
                ...subschema.properties,
                message: { type: 'string' },
              },
              required: [...(subschema.required as string[]), 'message'],
            }
          })
        } else if (schema.title !== 'PatchesSchema') {
          modifyTitle(schema)
        }

        return compile(schema, '', {
          bannerComment: '',
          style: { semi: false, singleQuote: true },
        })
      })
    )
  ).join('\n')

  // File formats, ordered by key to avoid big diffs each time they are regenerated
  const unordered = JSON.parse(addon.formatsFormats())
  const formats = Object.keys(unordered)
    .sort()
    .reduce((prev, key) => ({ ...prev, [key]: unordered[key] }), {})
  const json = JSON.stringify(formats, null, '  ')
  ts += `\nexport const FORMATS: Record<string, Format> = ${json}\n`

  fs.writeFileSync(path.join(__dirname, 'src', 'types.ts'), ts)
})()

// Replace `title` with `$id` or remove it. This is necessary because
// `json-schema-to-typescript` uses JSON schema `title` for the name of `interface`s
// whereas we use it for longer titles intended for user interfaces.
// See https://github.com/stencila/stencila/pull/929#issuecomment-842623228
function modifyTitle(value: unknown) {
  if (typeof value != 'object') return
  if (value === null) return
  const object = value as Record<string, unknown>

  for (const [key, child] of Object.entries(object)) {
    if (key == 'title') {
      if ('$id' in object) {
        object['title'] = object['$id']
      } else {
        delete object['title']
      }
    } else {
      modifyTitle(child)
    }
  }
}
