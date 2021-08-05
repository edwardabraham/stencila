import { EntityId } from '@reduxjs/toolkit'
import { Component, h, Host, Prop, State, Watch } from '@stencil/core'
import { DocumentEvent } from 'stencila'
import { CHANNEL } from '../../../../preload/channels'
import { state } from '../../../../renderer/store'
import { getProjectTheme } from '../../../../renderer/store/project/projectSelectors'
import { client } from '../../../client'

@Component({
  tag: 'app-document-preview',
  styleUrl: 'app-document-preview.css',
  shadow: true,
})
export class AppDocumentPreview {
  @Prop() documentId: EntityId

  @Watch('documentId')
  documentIdWatchHandler(newValue: string, prevValue: string) {
    if (newValue !== prevValue) {
      this.unsubscribeFromDocument(prevValue).then(() => {
        this.subscribeToDocument(newValue)
      })
    }
  }

  @State() previewContents: string

  private subscribeToDocument = (documentId = this.documentId) => {
    client.documents.preview(documentId).then(({ value: contents }) => {
      this.previewContents = contents
    })

    window.api.receive(CHANNEL.DOCUMENTS_PREVIEW, (event) => {
      const e = event as DocumentEvent
      if (
        e.document.id === documentId &&
        e.type === 'encoded' &&
        e.format?.name == 'html' &&
        e.content !== undefined
      ) {
        this.previewContents = e.content
      }
    })
  }

  private unsubscribeFromDocument = (documentId = this.documentId) => {
    window.api.removeAll(CHANNEL.DOCUMENTS_PREVIEW)
    return client.documents.unsubscribe({
      documentId,
      topics: ['encoded:html'],
    })
  }

  componentWillLoad() {
    this.subscribeToDocument()
  }

  disconnectedCallback() {
    this.unsubscribeFromDocument()
  }

  render() {
    return (
      <Host>
        <style>
          @import url('https://unpkg.com/@stencila/thema@latest/dist/themes/
          {getProjectTheme(state)}/styles.css');
        </style>

        <div
          class="app-document-preview"
          innerHTML={this.previewContents}
        ></div>
      </Host>
    )
  }
}
