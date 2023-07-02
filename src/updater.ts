import {
  checkUpdate,
  installUpdate,
} from '@tauri-apps/api/updater'
import { relaunch } from '@tauri-apps/api/process'
import { confirm } from '@tauri-apps/api/dialog'


export async function versionUpdate() {
  const message = useMessage()
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
    message.error(error as string)
  }
}
