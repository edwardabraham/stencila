/**
 * Generate Typescript language bindings.
 */

import fs from 'fs-extra'
import globby from 'globby'
import * as jstt from 'json-schema-to-typescript'
// @ts-ignore
import ObjectfromEntries from 'object.fromentries'
import path from 'path'
import tempy from 'tempy'
import * as schema from './schema'

/**
 * Run `build()` when this file is run as a Node script
 */
// eslint-disable-next-line @typescript-eslint/no-floating-promises
if (module.parent === null) build()

/**
 * Generate `dist/types.d.ts` from `built/*.schema.json`.
 */
async function build(): Promise<void> {
  // Ensure `*.schema.json` files are up to date
  await schema.build()

  // Our use of the `extends` keyword is different to that
  // used by `json-schema-to-typescript`. So we need to create
  // new set of `.schema.json` which do not have that keyword to avoid the clash.
  const files = await globby('*.schema.json', { cwd: 'built' })
  const temp = tempy.directory()
  await Promise.all(
    files.map(async file => {
      const schema = await fs.readJSON(path.join('built', file))
      delete schema.extends
      return fs.writeJSON(path.join(temp, file), schema)
    })
  )

  // Output `types.schema.json`
  // This 'meta' schema provides a list of type schemas as:
  //  - an entry point for the generation of Typescript type definitions
  //  - a lookup for all types for use in `util.ts` functions
  const types = {
    $schema: 'http://json-schema.org/draft-07/schema#',
    title: 'Types',
    properties: ObjectfromEntries(
      files.map(file => [
        file.replace('.schema.json', ''),
        { allOf: [{ $ref: file }] }
      ])
    ),
    required: files.map(file => file.replace('.schema.json', ''))
  }
  const typesFile = path.join(temp, 'types.schema.json')
  await fs.writeJSON(typesFile, types)

  const dist = path.join(__dirname, '..', 'dist')
  await fs.ensureDir(dist)

  // Generate the Typescript
  const options = {
    bannerComment: `/* eslint-disable */
/**
 * This file was automatically generated by ${__filename}.
 * Do not modify it by hand. Instead, modify the source \`.schema.yaml\` files
 * in the \`schema\` directory and run \`npm run build:ts\` to regenerate this file.
 */
 `
  }
  const ts = await jstt.compileFromFile(typesFile, options)
  await fs.writeFile(path.join(dist, 'types.d.ts'), ts)

  // Copy the JSON Schema files into `dist` too
  await Promise.all(
    files.map(async (file: string) =>
      fs.copy(path.join('built', file), path.join(dist, file))
    )
  )

  // Create an index.js for require.resolve('@stencila/schema') to work
  // properly in Encoda
  // TODO This won't be necessary when using tsc to compile an index.js
  await fs.writeFile(path.join(dist, 'index.js'), '\n')
}
