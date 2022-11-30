import { Attrs, Node, NodeSpec, ParseRule } from 'prosemirror-model'
import { EditorView } from 'prosemirror-view'
import StencilaMathBlock from '../../../nodes/math-block'
import { StencilaMathView, mathAttrs } from './math'

export function mathBlock(): NodeSpec {
  return {
    group: 'BlockContent',
    attrs: mathAttrs,
    parseDOM,
    toDOM,
  }
}

export class StencilaMathBlockView extends StencilaMathView<StencilaMathBlock> {
  constructor(node: Node, view: EditorView, getPos: () => number) {
    super(node, view, getPos, getAttrs, toDOM)
  }
}

const parseDOM: ParseRule[] = [
  {
    tag: 'stencila-math-block',
    getAttrs,
  },
]

function getAttrs(node: StencilaMathBlock): Attrs {
  return {
    id: node.id,
    mathLanguage: node.getAttribute('math-language'),
    code: node.querySelector('[slot=code]')?.innerHTML,
    errors: node.querySelector('[slot=errors]')?.innerHTML,
    mathml: node.querySelector('[slot=mathml]')?.innerHTML,
  }
}

function toDOM(node: Node) {
  const dom = document.createElement('stencila-math-block')
  dom.draggable = true
  dom.id = node.attrs.id
  dom.setAttribute('math-language', node.attrs.mathLanguage)

  const code = document.createElement('pre')
  code.slot = 'code'
  code.innerHTML = node.attrs.code
  code.contentEditable = 'false'
  dom.appendChild(code)

  const errors = document.createElement('div')
  errors.slot = 'errors'
  errors.innerHTML = node.attrs.errors
  errors.contentEditable = 'false'
  dom.appendChild(errors)

  const mathml = document.createElement('div')
  mathml.slot = 'mathml'
  mathml.innerHTML = node.attrs.mathml
  mathml.contentEditable = 'false'
  dom.appendChild(mathml)

  return { dom }
}