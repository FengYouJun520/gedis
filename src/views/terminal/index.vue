<script setup lang="ts">
import { TabsProps, useTabs } from '@/store/tabs'
import { EventMessage } from '@/types/vue-web-terminal'
import { invoke } from '@tauri-apps/api'
import Terminal from 'vue-web-terminal'

interface TerminalProps {
  tabItem: TabsProps
}

const props = defineProps<TerminalProps>()
const db = ref(props.tabItem.db)

const argsRegex = /[\s*]|"(.*)"/
const tabsState = useTabs()

const handleInitComplete = () => {
  Terminal.$api.focus(props.tabItem.id)
}

const onExecCmd = async (key: string, command: string, success: EventMessage, failed: EventMessage) => {
  const args = command.split(argsRegex).filter(arg => arg && arg !== '')

  if (key === 'exit') {
    await invoke('dis_connection', { id: props.tabItem.id })
    tabsState.removeTab(`${props.tabItem.id}-${props.tabItem.db}`)
    return
  }

  if (args.length > 1 && args[0].toLowerCase() === 'select') {
    db.value = parseInt(args[1])
  }

  invoke<string| any[] | any[][]>('terminal', {
    id: props.tabItem.id,
    db: unref(db),
    args,
  }).then(res => {
    if (typeof res === 'string') {
      success({
        type: 'normal',
        content: res,
      })
    } else {
      success({
        type: 'normal',
        content: res[0],
      })
      for (const item of res[1] as string[]) {
        success({
          type: 'normal',
          content: item,
        })
      }
    }
  })
    .catch(error => {
      failed(error)
    })

  Terminal.$api.focus(props.tabItem.id)
}
</script>

<template>
  <Terminal
    :name="tabItem.id"
    :title="tabItem.name"
    :context="tabItem.name"
    :show-header="false"
    :auto-help="false"
    :enable-example-hint="false"
    :init-complete="handleInitComplete"
    @exec-cmd="onExecCmd"
  >
    <template #header />
  </Terminal>
</template>

<style lang="css" scoped>
:deep(.t-window) {
  background-color: var(--el-bg-color-overlay);
}
</style>
