import {
  checkUpdate,
  installUpdate,
  onUpdaterEvent,
} from '@tauri-apps/api/updater'
import { relaunch } from '@tauri-apps/api/process'
import { confirm } from '@tauri-apps/api/dialog'


export async function useUpdater(showUptodate = false, showError = false) {
  const unlisten = await onUpdaterEvent(({ error, status }) => {
    if (status === 'UPTODATE' && showUptodate) {
      confirm('目前是最新版', {
        title: '更新通知',
        type: 'info',
      })
    }
    if (error && showError) {
      confirm(error!, {
        title: '更新错误',
        type: 'error',
      })
    }
  })

  try {
    const { shouldUpdate, manifest } = await checkUpdate()

    if (shouldUpdate) {
      await confirm(`有新版本可用：${manifest?.version}, ${manifest?.date}, ${manifest?.body}，
是否现在更新？`, {
        type: 'info',
        title: '版本更新',
        okLabel: '现在更新',
        cancelLabel: '稍后更新',
      })

      // Install the update. This will also restart the app on Windows!
      await installUpdate()

      // On macOS and Linux you will need to restart the app manually.
      // You could use this step to display another confirmation dialog.
      await relaunch()
    }
  } catch (error) {
    console.error(error)
  } finally {
    unlisten()
  }
}
