import { install, list, schema, uninstall, upgrade } from './plugins'

describe.skip('plugins', () => {
  test('schema', () => {
    expect(schema()).toEqual(
      expect.objectContaining({
        $schema: 'https://json-schema.org/draft/2019-09/schema',
        $id: 'Plugin',
        title: expect.stringMatching(/^Description of a plugin$/),
        type: 'object',
        properties: expect.objectContaining({
          name: { description: 'The name of the plugin', type: 'string' },
        }),
      })
    )
  })

  test('list', () => {
    expect(list()).toEqual(expect.arrayContaining([]))
  })

  test.skip('install', () => {
    expect(install('javascript')).toEqual(expect.arrayContaining([]))
  })

  test('uninstall', () => {
    expect(uninstall('javascript')).toEqual(expect.arrayContaining([]))
  })

  test.skip('upgrade', () => {
    expect(upgrade('javascript')).toEqual(expect.arrayContaining([]))
  })
})
