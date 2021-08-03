import { ipcRenderer } from 'electron'
import log from 'electron-log'
import { IpcRendererAPI } from '../preload/types'
import { Channel, Handler, isChannel } from './channels'

export const apis: IpcRendererAPI = {
  // TODO: Refine type definitions to enable guarding against valid channels
  invoke: ipcRenderer.invoke,
  send: (channel: Channel, data: unknown) => {
    if (isChannel(channel)) {
      ipcRenderer.send(channel, data)
    }
  },
  receive: (channel: Channel, func: Handler) => {
    if (isChannel(channel)) {
      // Deliberately strip event as it includes `sender`
      ipcRenderer.on(channel, (_event, ...args) => func(...args))
    }
  },
  remove: (channel: Channel, func: Handler) => {
    if (isChannel(channel)) {
      // Deliberately strip event as it includes `sender`
      ipcRenderer.removeListener(channel, (_event, ...args) => func(...args))
    }
  },
  removeAll: (channel: Channel) => {
    ipcRenderer.removeAllListeners(channel)
  },
  log,
}
