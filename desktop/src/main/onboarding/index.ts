import { ipcMain } from 'electron'
import { CHANNEL } from '../../preload'
import { removeChannelHandlers } from '../utils/handler'
import { ONBOARDING_CHANNEL } from './channels'
import { closeOnboardingWindow, openOnboardingWindow } from './window'

export const registerOnboardingHandlers = () => {
  ipcMain.handle(CHANNEL.OPEN_ONBOARDING_WINDOW, async () => {
    return openOnboardingWindow()
  })

  ipcMain.handle(CHANNEL.CLOSE_ONBOARDING_WINDOW, async () => {
    return closeOnboardingWindow()
  })
}

export const removeOnboaringHandlers = () => {
  removeChannelHandlers(ONBOARDING_CHANNEL)
}