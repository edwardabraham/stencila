import { Attrs, Node } from 'prosemirror-model'
import { NodeSelection, TextSelection } from 'prosemirror-state'
import { EditorView, NodeView } from 'prosemirror-view'
import { Patch, Then } from '../../../../types'
import StencilaEntity from '../../../nodes/entity'

export const entityAttrs = {
  id: { default: null },
}

export class StencilaEntityView<Type extends StencilaEntity>
  implements NodeView
{
  node: Node

  view: EditorView

  getPos: () => number

  getAttrs: (dom: Type | string) => Attrs | false

  toDOM: (node: Node) => { dom: HTMLElement; contentDOM?: HTMLElement }

  dom: Type

  contentDOM?: HTMLElement

  constructor(
    node: Node,
    view: EditorView,
    getPos: () => number,
    getAttrs: (dom: Type | string) => Attrs | false,
    toDOM: (node: Node) => { dom: HTMLElement; contentDOM?: HTMLElement }
  ) {
    this.node = node
    this.view = view
    this.getPos = getPos
    this.getAttrs = getAttrs
    this.toDOM = toDOM

    const { dom, contentDOM } = toDOM(node)
    this.dom = dom as Type
    this.contentDOM = contentDOM

    // Listen for node select/deselect events

    this.dom.addEventListener('stencila-select', (event) => {
      const state = this.view.state
      this.view.dispatch(
        state.tr.setSelection(NodeSelection.create(state.doc, this.getPos()))
      )
      event.stopPropagation()
    })

    this.dom.addEventListener('stencila-deselect', (event) => {
      const state = this.view.state
      this.view.dispatch(
        state.tr.setSelection(
          // TODO: getPos() + 1 will select the first child in the content of node
          // (if it has any) but it might be better to place cursor at the end on the
          // content?
          TextSelection.create(state.doc, this.getPos() + 1)
        )
      )
      event.stopPropagation()
    })

    // When the Web Component emits a patch, dispatch it as a ProseMirror transaction
    // and stop the patch bubbling up to the window's patch handler
    this.dom.addEventListener(
      'stencila-document-patch',
      (event: CustomEvent) => {
        const { patch } = event.detail
        this.dispatchPatch(patch)
        event.stopPropagation()
      }
    )
  }

  /**
   * Show the node as selected
   */
  selectNode() {
    this.dom.selected = true
  }

  /**
   * Show the node as not selected
   */
  deselectNode() {
    this.dom.selected = false
  }

  /**
   * Prevent the editor view from trying to handle some or all DOM events that
   * bubble up from the node view
   *
   * This prevents all events apart from drag events (to enable drag and drop).
   * Without this, a lot of things break, in particular the toggling of node selection
   * and backspacing in <input>s. However, it is not clear if this blanket banning
   * affects the editing of ProseMirror content. A more selective approach may be
   * required (e.g. only ignoring events on inputs).
   *
   * @see https://discuss.prosemirror.net/t/creating-a-custom-node-with-inline-input/1282/5
   */
  stopEvent(event: Event) {
    return !event.type.startsWith('drag')
  }

  /**
   * Dispatch a Stencila document `Patch` as a ProseMirror `Transaction`
   *
   * This is done so that undo/redo works at the document level.
   * The patch is added as metadata to the transaction so that it can be
   * sent to the Stencila server rather than attempting to to generate a patch
   * from it.
   */
  dispatchPatch(patch: Patch) {
    // TODO check that the patch.target is the same as the node.id
    const transaction = this.view.state.tr
    for (const op of patch.ops) {
      switch (op.type) {
        case 'Replace':
          const attribute = op.address[0]
          if (typeof attribute !== 'string' || op.address.length > 1) {
            console.error(
              `Expected replace operation address to have single name slot; got ${op.address}`
            )
          } else {
            transaction.setNodeAttribute(this.getPos(), attribute, op.value)
          }
          break
        default:
          // TODO: implement handling of other types of operations
          console.warn(`Operation ${op.type} is not handled`, op)
      }
    }
    transaction.setMeta('stencila-document-patch', patch)
    this.view.dispatch(transaction)
  }

  /**
   * Handle a mutation in the view
   *
   * Node views that extend this class must define a `handleMutation` method
   * to dispatch a ProseMirror transaction for non-editable, derived properties
   * e.g. `Executable.errors` & `CodeExecutable.outputs` that are received from the server
   * as patches. The transaction should not trigger a patch to be sent to the server.
   * Do this using:
   *
   *    transaction.setMeta('stencila-document-patch', false)
   *
   * Without doing this derived properties will not get carried across when
   * a node is copy+pasted to a new node.
   */
  handleMutation(mutation: MutationRecord): void {}

  /**
   * Whether to ignore mutations in the node
   *
   * This is necessary to because the Web Component performs async mutations on the
   * DOM within the view. See https://discuss.prosemirror.net/t/asynchronous-dom-mutation-and-customnodeview-ignoremutation/3588/2
   */
  ignoreMutation(mutation: MutationRecord): boolean {
    this.handleMutation(mutation)
    return true
  }

  /**
   * Update the view
   *
   * If this method returns `false` it means the view could not be updated and so needs
   * to be replaced with a new view.
   *
   * This currently returns `true` in most cases because we do not want a rerender, and
   * thus a loss of cursor position on code editors and other inputs within the
   * Web Component.
   */
  update(node: Node): boolean {
    // If the node has changed type then, no, we can't update the current view
    // so return false
    if (node.type !== this.node.type) {
      return false
    }

    // TODO: this needs more sophisticated checks to see if the change came from
    // inside the component or not (e.g. undo was pressed and ProseMirror wants to
    // update the node to a previous stage)

    this.node = node
    return true
  }
}