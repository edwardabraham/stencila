import { app, BrowserWindow, protocol } from 'electron'
import installExtension, { REDUX_DEVTOOLS } from 'electron-devtools-installer'
import { main, prepare } from './main'
import { requestHandler, scheme } from './main/app-protocol'
import { openLauncherWindow } from './main/launcher/window'
import * as localProtocol from './main/local-protocol'
import { openOnboardingWindow } from './main/onboarding/window'
import { isFirstLaunch, setFirstLaunchState } from './main/utils/firstLaunch'
import { isDevelopment } from './preload/utils/env'

prepare()

// Handle creating/removing shortcuts on Windows when installing/uninstalling.
if (require('electron-squirrel-startup')) {
  app.quit()
}

if (process.platform === 'linux') {
  // This is necessary to avoid UI rendering glitches on Ubuntu, and possibly other Linux distributions.
  // TODO: Investigate the root cause and see if the OS targeting can be reduced to only apply to
  // specific OS distributions where needed.
  app.disableHardwareAcceleration()
}

const createMainWindow = (): void => {
  /* eng-disable PROTOCOL_HANDLER_JS_CHECK */
  protocol.registerBufferProtocol(scheme, requestHandler)

  protocol.registerFileProtocol(
    localProtocol.scheme,
    localProtocol.requestHandler
  )

  // If app is launched for the first time, show onboarding flow
  if (isFirstLaunch()) {
    openOnboardingWindow()
    setFirstLaunchState(false)
  } else {
    openLauncherWindow()
  }
}

protocol.registerSchemesAsPrivileged([
  {
    scheme: scheme,
    privileges: {
      standard: true,
      secure: true,
    },
  },
])

if (isDevelopment) {
  app.whenReady().then(() => {
    installExtension(REDUX_DEVTOOLS, {
      loadExtensionOptions: { allowFileAccess: true },
      forceDownload: false,
    })
      .then((name: string) => console.log(`Added Extension: ${name}`))
      .catch((err: unknown) =>
        console.log('Failed to install dev tools extension: ', err)
      )
  })
}

// This method will be called when Electron has finished
// initialization and is ready to create browser windows.
// Some APIs can only be used after this event occurs.
app.on('ready', createMainWindow)

// Quit when all windows are closed, except on macOS. There, it's common
// for applications and their menu bar to stay active until the user quits
// explicitly with Cmd + Q.
app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit()
  }
})

app.on('activate', () => {
  // On OS X it's common to re-create a window in the app when the
  // dock icon is clicked and there are no other windows open.
  if (BrowserWindow.getAllWindows().length === 0) {
    createMainWindow()
  }
})

// In this file you can include the rest of your app's specific main process
// code. You can also put them in separate files and import them here.
main()
