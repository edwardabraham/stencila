import pkg from '../../package.json'
import {
  jsonLdUrl,
  jsonLdContext,
  jsonLdTermUrl,
  jsonLdTermName,
} from './jsonld'

test('jsonLdUrl', () => {
  const expectedBase = `https://schema.stenci.la/v${
    pkg.version.split('.')[0]
  }/jsonld/`

  expect(jsonLdUrl()).toMatch(expectedBase)
  expect(jsonLdUrl('CodeChunk')).toMatch(expectedBase + 'CodeChunk')
  expect(jsonLdUrl('outputs')).toMatch(expectedBase + 'outputs')
})

test('jsonLdContext', () => {
  expect(jsonLdContext()).toHaveProperty('stencila')
  expect(jsonLdContext().stencila).toEqual(jsonLdUrl())
})

test('jsonLdTermUrl', () => {
  const stencilaUrl = jsonLdUrl()
  expect(jsonLdTermUrl('CodeChunk')).toEqual(stencilaUrl + 'CodeChunk')
  expect(jsonLdTermUrl('outputs')).toEqual(stencilaUrl + 'outputs')

  expect(jsonLdTermUrl('Article')).toEqual('https://schema.org/Article')
  expect(jsonLdTermUrl('authors')).toEqual('https://schema.org/author')

  expect(jsonLdTermUrl('foo')).toBeUndefined()
})

test('jsonLdTermName', () => {
  const stencilaUrl = jsonLdUrl()
  expect(jsonLdTermName(stencilaUrl + 'CodeChunk')).toEqual('CodeChunk')
  expect(jsonLdTermName(stencilaUrl + 'outputs')).toEqual('outputs')

  expect(jsonLdTermName('https://schema.org/Article')).toEqual('Article')
  expect(jsonLdTermName('https://schema.org/author')).toEqual('authors')
})
