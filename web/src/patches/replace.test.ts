import {
  applyReplace,
  applyReplaceOption,
  applyReplaceString,
  applyReplaceVec,
} from './replace'

test('applyReplaceOption', () => {
  const elem = document.createElement('div')
  elem.innerHTML = '<p slot="property">One</p>'
  expect(elem.querySelector('[slot="property"]')?.innerHTML).toEqual('One')

  applyReplaceOption(elem, 'property', 1, '<p slot="property">Two</p>')
  expect(elem.querySelector('[slot="property"]')?.innerHTML).toEqual('Two')

  applyReplaceOption(elem, 'property', 1, '<p slot="property">Three</p>')
  expect(elem.querySelector('[slot="property"]')?.innerHTML).toEqual('Three')

  expect(() => applyReplaceOption(elem, 1, 1, '')).toThrow(
    /Expected string slot/
  )
  expect(() => applyReplaceOption(elem, '', 100, '')).toThrow(
    /Unexpected replace items 100/
  )
  expect(() => applyReplaceOption(elem, 'foo', 1, '')).toThrow(
    /Unable to resolve slot 'foo'/
  )
})

test('applyReplaceVec', () => {
  const elem = document.createElement('ol')
  elem.innerHTML = '<li>1</li><li>2</li><li>3</li><li>4</li><li>5</li>'

  applyReplaceVec(elem, 1, 1, '<li>two</li>')
  expect(elem.innerHTML).toEqual(
    '<li>1</li><li>two</li><li>3</li><li>4</li><li>5</li>'
  )

  applyReplaceVec(elem, 2, 3, '<li>three,four</li>')
  expect(elem.innerHTML).toEqual('<li>1</li><li>two</li><li>three,four</li>')

  applyReplaceVec(elem, 0, 1, '<li>first</li>')
  expect(elem.innerHTML).toEqual(
    '<li>first</li><li>two</li><li>three,four</li>'
  )

  expect(() => applyReplaceVec(elem, 'string', 1, '')).toThrow(
    /Expected number slot/
  )
  expect(() => applyReplaceVec(elem, -1, 1, '')).toThrow(
    /Unexpected replace slot '-1'/
  )
  expect(() => applyReplaceVec(elem, 42, 1, '')).toThrow(
    /Unexpected replace slot '42'/
  )
})

test('applyReplaceString', () => {
  const node = document.createTextNode('abc🎁de')

  applyReplaceString(node, 0, 1, 'x🏳️‍🌈')
  expect(node.textContent).toEqual('x🏳️‍🌈bc🎁de')

  applyReplaceString(node, 1, 6, 'yz')
  expect(node.textContent).toEqual('xyz')

  expect(() => applyReplaceString(node, 'string', 1, '')).toThrow(
    /Expected number slot/
  )
  expect(() => applyReplaceString(node, -1, 1, '')).toThrow(
    /Unexpected replace slot '-1'/
  )
  expect(() => applyReplaceString(node, 42, 1, '')).toThrow(
    /Unexpected replace slot '42'/
  )
  expect(() => applyReplaceString(node, 0, 100, '')).toThrow(
    /Unexpected replace items 100/
  )
})

test('applyReplace', () => {
  // Start with `Article` with one paragraph with some content
  document.body.innerHTML =
    '<article slot="root"><div slot="content"><p>' +
    'One <strong>two</strong> three.' +
    '</p></div></article>'

  // Replace one character of 'two'
  applyReplace({
    type: 'Replace',
    address: ['content', 0, 'content', 1, 'content', 0, 1],
    items: 1,
    html: '-',
  })
  expect(document.body).toMatchInlineSnapshot(`
<body>
  <article
    slot="root"
  >
    <div
      slot="content"
    >
      <p>
        One 
        <strong>
          t-o
        </strong>
         three.
      </p>
    </div>
  </article>
</body>
`)

  // Replace the `Strong` and the previous word
  applyReplace({
    type: 'Replace',
    address: ['content', 0, 'content', 0],
    items: 2,
    html: 'one, two',
  })
  expect(document.body).toMatchInlineSnapshot(`
<body>
  <article
    slot="root"
  >
    <div
      slot="content"
    >
      <p>
        one, two
         three.
      </p>
    </div>
  </article>
</body>
`)

  // Replace the article content
  applyReplace({
    type: 'Replace',
    address: ['content'],
    items: 1,
    html: '<div slot="content"><p>Hello</p></div>',
  })
  expect(document.body).toMatchInlineSnapshot(`
<body>
  <article
    slot="root"
  >
    <div
      slot="content"
    >
      <p>
        Hello
      </p>
    </div>
  </article>
</body>
`)
})
