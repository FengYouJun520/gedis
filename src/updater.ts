import {
  checkUpdate,
  installUpdate,
  onUpdaterEvent,
} from '@tauri-apps/api/updater'
import { relaunch } from '@tauri-apps/api/process'
import { confirm, message } from '@tauri-apps/api/dialog'


export async function useUpdater(showUptodate = false, showError = false) {
  const unlisten = await onUpdaterEvent(({ error, status }) => {
    if (status === 'UPTODATE' && showUptodate) {
      message('目前已是最新版本', {
        title: '更新通知',
        type: 'info',
      })
    }
    if (error && showError) {
      message(error, {
        title: '更新错误',
        type: 'error',
      })
    }
  })

  try {
    const { shouldUpdate, manifest } = await checkUpdate()

    if (shouldUpdate) {
      const ok = await confirm(`有新版本可用：${manifest?.version}, 
更新时间：${manifest?.date}
${manifest?.body}
是否现在更新？`, {
        type: 'info',
        title: '版本更新',
        okLabel: '现在更新',
        cancelLabel: '稍后更新',
      })

      if (ok) {
        // Install the update. This will also restart the app on Windows!
        await installUpdate()

        // On macOS and Linux you will need to restart the app manually.
        // You could use this step to display another confirmation dialog.
        await relaunch()
      }
    }
  } catch (error) {
    console.error(error)
  } finally {
    unlisten()
  }
}
