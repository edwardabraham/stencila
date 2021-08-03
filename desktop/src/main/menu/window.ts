import { MenuItemConstructorOptions } from 'electron'
import { openLauncherWindow } from '../launcher/window'
import { showLogs } from '../logging/window'
import { isMac } from './utils'

export const baseWindowMenu: MenuItemConstructorOptions = {
  label: 'Window',
  submenu: [
    { role: 'minimize' },
    { role: 'zoom' },
    { type: 'separator' },
    {
      label: 'Launcher',
      accelerator: 'Shift+CommandOrControl+1',
      click: () => {
        openLauncherWindow()
      },
    },
    ...(isMac
      ? [
          { type: 'separator' as const },
          { role: 'front' as const },
          { type: 'separator' as const },
          { role: 'window' as const },
        ]
      : [{ role: 'close' as const }]),
    { type: 'separator' },
    {
      label: 'Advanced',
      submenu: [
        {
          label: 'Debug Logs',
          click: () => {
            showLogs()
          },
        },
        { type: 'separator' },
        { role: 'reload' },
        { role: 'forceReload' },
        { role: 'toggleDevTools' },
      ],
    },
  ],
}
