import { createSlice, EntityId, PayloadAction } from '@reduxjs/toolkit'
import { array as A, option as O } from 'fp-ts'
import { pipe } from 'fp-ts/function'
import { Document } from 'stencila'
import {
  DocumentPane,
  NormalizedDocumentPaneStore,
  PaneModule,
} from './documentPaneTypes'

const initialState: NormalizedDocumentPaneStore = {
  activePane: O.none,
  entities: {
    panes: {},
    layouts: {},
    views: {},
  },
  ids: [],
}

export const makeLayoutId =
  (paneId: EntityId) =>
  (viewId: EntityId): string =>
    `${paneId}-${viewId}`

export const documentPaneSlice = createSlice({
  name: 'documentPanes',
  initialState: initialState,
  reducers: {
    createPane: (state, { payload }: PayloadAction<{ paneId: EntityId }>) => {
      state.activePane = O.some(payload.paneId)
      state.entities.panes[payload.paneId] = {
        id: payload.paneId,
        activeView: O.none,
        views: [],
      }
      state.ids = [...state.ids, payload.paneId]
    },
    updatePane: (
      state,
      {
        payload,
      }: PayloadAction<{ id: EntityId; changes: Partial<DocumentPane> }>
    ) => {
      const prevPane = state.entities.panes[payload.id]
      if (prevPane) {
        state.entities.panes[payload.id] = {
          ...prevPane,
          ...payload.changes,
        }
      }
    },
    setPaneModuleVisibility: (
      state,
      {
        payload,
      }: PayloadAction<{
        layoutId: EntityId
        module: PaneModule
        isVisible: boolean
      }>
    ) => {
      const prevLayout = state.entities.layouts[payload.layoutId]
      const prevModules = prevLayout?.modules
      if (prevLayout && prevModules) {
        // If module is not already part of the pane, add it
        if (payload.isVisible && !prevModules.includes(payload.module)) {
          const newModules: PaneModule[] = [...prevModules, payload.module]

          state.entities.layouts[payload.layoutId]!.modules = newModules
        }
        // Otherwise remove the module from the pane
        else if (!payload.isVisible) {
          const newModules: PaneModule[] = prevModules.filter(
            (module) => module !== payload.module
          )

          state.entities.layouts[payload.layoutId]!.modules = newModules
        }
      }
    },
    addDocToPane: (
      state,
      { payload }: PayloadAction<{ paneId: EntityId; doc: Document }>
    ) => {
      const pane = state.entities.panes[payload.paneId]
      if (pane) {
        if (!pane.views.includes(payload.doc.id)) {
          pane.views = [...pane.views, payload.doc.id]
          state.entities.views[payload.doc.id] = payload.doc

          const modules: PaneModule[] = []

          if (!payload.doc.format.binary) {
            modules.push('editor')
          }

          if (payload.doc.previewable) {
            modules.push('preview')
          }

          const moduleCount = modules.length
          const layoutId = makeLayoutId(payload.paneId)(payload.doc.id)

          state.entities.layouts[layoutId] = {
            id: layoutId,
            orientation: 'horizontal',
            modules,
            sizes: A.makeBy(moduleCount, () => 1 / moduleCount),
          }
        }
        pane.activeView = O.some(payload.doc.id)
      }
      return state
    },
    updateDoc: (state, { payload }: PayloadAction<{ doc: Document }>) => {
      state.entities.views[payload.doc.id] = payload.doc
      return state
    },
    patchDoc: (
      state,
      {
        payload,
      }: PayloadAction<{
        doc: Partial<Omit<Document, 'id'>> & { id: EntityId }
      }>
    ) => {
      const id = payload.doc.id.toString()
      const currentState = state.entities.views[id]

      if (currentState) {
        state.entities.views[id] = {
          ...currentState,
          ...payload.doc,
          id,
        }
      }

      return state
    },
    removeDocFromPane: (
      state,
      { payload }: PayloadAction<{ paneId: EntityId; docId: EntityId }>
    ) => {
      const pane = state.entities.panes[payload.paneId]

      if (pane) {
        const docIndex = pane.views.indexOf(payload.docId)

        // Remove document from list
        if (pane.views.includes(payload.docId)) {
          pane.views = pipe(
            pane.views,
            A.deleteAt(docIndex),
            O.getOrElse<EntityId[]>(() => [])
          )
        }

        // If document being closed is not the currently active document,
        // change focus to the closest tab
        if (
          pipe(
            pane.activeView,
            O.map((doc) => doc === payload.docId),
            O.getOrElse(() => false)
          )
        ) {
          pane.activeView = pipe(
            A.lookup(docIndex)(pane.views),
            O.alt(() => A.lookup(docIndex - 1)(pane.views))
          )
        }

        // TODO: If document is not present in any other panes, remove from store
      }

      return state
    },
    nextDocInPane: (
      state,
      { payload }: PayloadAction<{ paneId: EntityId, direction: 'next' | 'previous' }>
    ) => {
      const paneState = state.entities.panes[payload.paneId]
      const directionIndex = payload.direction === 'next' ? 1 : -1

      if (paneState) {
        pipe(
          paneState.activeView,
          O.map((view) => paneState.views.indexOf(view)),
          O.map((activeViewIndex) => paneState.views[activeViewIndex + directionIndex]),
          O.map(O.fromNullable),
          O.flatten,
          O.alt(() => 
            // If there isn't a next pane, loop through the list
            payload.direction === 'next' ? A.head(paneState.views) : A.last(paneState.views)
          ),
          O.map((nextViewId) => {
            console.log('nextViewId: ', nextViewId)
            paneState.activeView = O.some(nextViewId)
            return nextViewId
          })
        )
      }
    },
  },
})

export const documentPaneActions = documentPaneSlice.actions
export type DocumentPaneActions = typeof documentPaneActions
