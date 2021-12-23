import fs from 'fs'
import path from 'path'
import tmp from 'tmp'
import {
  addSource,
  graph,
  importSource,
  open,
  removeSource,
  schemas,
  subscribe,
  write,
} from './projects'
import { FileEvent, GraphEvent, ProjectEvent } from './types'

/**
 * Get the path to one of the project fixtures
 */
function fixture(folder: string) {
  return path.normalize(
    path.join(__dirname, '..', '..', 'fixtures', 'projects', folder)
  )
}

/**
 * Wait for a bit (usually for events), longer on CI (because that seems necessary).
 */
async function delay(milliseconds: number) {
  await new Promise((resolve) =>
    setTimeout(resolve, milliseconds * (process.env.CI ? 4 : 1))
  )
}

test('schema', () => {
  expect(schemas()[0]).toEqual(
    expect.objectContaining({
      $schema: 'https://json-schema.org/draft/2019-09/schema',
      $id: 'Project',
      title: expect.stringMatching(/^Details of a project$/),
      type: 'object',
      properties: expect.objectContaining({
        name: expect.objectContaining({
          description: 'The name of the project',
        }),
      }),
    })
  )
})

test('open: empty', () => {
  let folder = fixture('empty')
  expect(open(folder)).toEqual(
    expect.objectContaining({
      path: folder,
      name: 'empty',
      theme: 'stencila',
    })
  )
})

test('open: manifest', () => {
  let folder = fixture('manifest')
  expect(open(folder)).toEqual(
    expect.objectContaining({
      path: folder,
      name: 'A project with a project.json file',
      theme: 'wilmore',
      mainPath: path.join(folder, 'my-main-file.md'),
      files: expect.objectContaining({
        [path.join(folder, 'project.json')]: expect.objectContaining({
          name: 'project.json',
          format: expect.objectContaining({ extension: 'json' }),
        }),
        [path.join(folder, 'my-main-file.md')]: expect.objectContaining({
          name: 'my-main-file.md',
          format: expect.objectContaining({ extension: 'md' }),
        }),
      }),
    })
  )
})

/**
 * Test of a workflow involving opening and modifying a project
 * and receiving events for: updated properties and graph and
 * any file events within the project
 */
test('workflow: open and modify', async () => {
  const folder = tmp.dirSync().name
  let projectEvents: ProjectEvent[] = []
  let fileEvents: FileEvent[] = []
  let graphEvents: GraphEvent[] = []

  // Open the project
  const project = open(folder)
  expect(project).toEqual(
    expect.objectContaining({
      path: folder,
      theme: 'stencila',
    })
  )

  // Subscribe to the project
  subscribe(folder, ['props'], (_topic, event) =>
    projectEvents.push(event as ProjectEvent)
  )
  subscribe(folder, ['files'], (_topic, event) =>
    fileEvents.push(event as FileEvent)
  )
  subscribe(folder, ['graph'], (_topic, event) =>
    graphEvents.push(event as GraphEvent)
  )

  // Wait for events to propagate before clearing event arrays
  await delay(500)
  projectEvents = []
  fileEvents = []
  graphEvents = []

  // Modify the project.json file on disk
  fs.writeFileSync(
    path.join(folder, 'project.json'),
    JSON.stringify({
      theme: 'wilmore',
    })
  )
  // Wait for debounced filesystem events to propagate
  await delay(500)

  expect(projectEvents).toEqual(
    expect.arrayContaining([
      expect.objectContaining({
        type: 'updated',
        project: expect.objectContaining({
          path: folder,
          theme: 'wilmore',
        }),
      }),
    ])
  )
  expect(fileEvents).toEqual(
    expect.arrayContaining([
      expect.objectContaining({
        type: 'created',
        path: path.join(folder, 'project.json'),
      }),
    ])
  )
  expect(graphEvents).toEqual(
    expect.arrayContaining([
      expect.objectContaining({
        type: 'updated',
        graph: expect.objectContaining({
          nodes: expect.arrayContaining([]),
          edges: expect.arrayContaining([]),
        }),
      }),
    ])
  )
})

/**
 * Test of a workflow involving adding and removing sources
 *
 * Skipped because getting a 403 when attempting to
 * import source.
 */
if (!process.env.CI) {
  test.skip('workflow: sources', async () => {
    const folder = tmp.dirSync().name
    open(folder)

    addSource(folder, 'elife://60912', 'article.xml', 'source-1')

    importSource(folder, 'source-1')
    expect(fs.existsSync(path.join(folder, 'article.xml')))

    removeSource(folder, 'source-1')
  })
}

test('graph', async () => {
  const folder = tmp.dirSync().name
  open(folder)
  expect(graph(folder, 'dot')).toMatch(/^\s*digraph/)
})
