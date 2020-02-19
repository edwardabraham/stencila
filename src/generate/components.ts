import fs from 'fs'
import globby from 'globby'
import path from 'path'

if (module.parent === null) {
  const [func, arg1] = process.argv.slice(2)
  if (func === 'create') create(arg1)
  else if (func === 'update') update()
  else console.error(`Unrecognised function: ${func}`)
}

/**
 * Create a new component folder in `../components/`.
 *
 * Run using `npm run create:component -- <name-of-component>`.
 *
 * Creates the folder and populates with the necessary files
 * containing placeholder content.
 */
function create(name?: string): void {
  // Check that a name has been supplied
  if (name === undefined) {
    console.log(`You must supply an component name`)
    process.exit(1)
  }

  const componentDir = path.join(__dirname, '..', 'components', name)

  // Check that the component does not already exist
  if (fs.existsSync(componentDir)) {
    console.log(`Addon "${name}" already exists: ${componentDir}`)
    process.exit(1)
  } else {
    fs.mkdirSync(componentDir)
  }

  // Create necessary files
  fs.writeFileSync(
    path.join(componentDir, 'README.md'),
    `# ${name[0].toUpperCase()}${name.slice(1)} component

<!-- Add a description of your component and notes for contributors. -->\n`
  )

  fs.writeFileSync(
    path.join(componentDir, 'index.ts'),
    `// Do any initialization that your component requires here.\n\nexport {}\n`
  )

  fs.writeFileSync(
    path.join(componentDir, 'styles.css'),
    `/* Add your component's styles to this file */\n`
  )

  // Update `components.ts` etc
  update(false)
}

/**
 * Generate `../components/components.ts`.
 *
 * Run using `npm run update:components`.
 *
 * If an component has a `update.{js|ts}` file that will be run
 * also (if `all` is `true`).
 */
function update(all = true): void {
  const componentsDir = path.join(__dirname, '..', 'components')

  // Get the list of components
  const components = globby.sync('*', {
    onlyDirectories: true,
    cwd: componentsDir
  })

  components.forEach(async component => {
    const componentDir = path.join(componentsDir, component)

    // Check each component has the necessary files
    ;['README.md', 'styles.css', ['index.js', 'index.ts']].forEach(file => {
      const files = globby.sync(file, {
        onlyFiles: true,
        cwd: componentDir
      })
      if (files.length !== 1) {
        console.error(`Theme "${component}" must have one "${file}" file`)
        process.exit(1)
      }
    })

    // Run the update script if it is present
    if (all) {
      try {
        await import(path.join(componentDir, 'update'))
      } catch (error) {
        if (!/^Cannot find module/.test(error.message)) {
          console.error(error)
          process.exit(1)
        }
      }
    }
  })

  // Write all components to `components.ts`
  fs.writeFileSync(
    path.join(__dirname, '..', 'components', 'components.ts'),
    `// Generated by generate/${path.basename(__filename)}. Do not edit.

/**
 * Map of component Javascript modules
 */
export const components: {
  ${components.map(component => `${component}: Promise<unknown>`).join('\n  ')}
} = {
  ${components.map(component => `${component}: import('./${component}')`).join(',\n  ')}
}\n`
  )
}
