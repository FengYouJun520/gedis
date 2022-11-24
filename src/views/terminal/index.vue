<script setup lang="ts">
import { TabsProps, useTabs } from '@/store/tabs'
import { EventMessage } from '@/types/vue-web-terminal'
import { invoke } from '@tauri-apps/api'
import Terminal from 'vue-web-terminal'

interface TerminalProps {
  tabItem: TabsProps
}

const props = defineProps<TerminalProps>()

const tabsState = useTabs()

const handleInitComplete = () => {
  Terminal.$api.focus(props.tabItem.id)
}

const onExecCmd = async (key: string, command: string, success: EventMessage, failed: EventMessage) => {
  if (key === 'exit') {
    await invoke('dis_connection', { id: props.tabItem.id })
    tabsState.removeTab(`${props.tabItem.id}-${props.tabItem.db}`)
    return
  }

  invoke<string>('terminal', {
    id: props.tabItem.id,
    db: props.tabItem.db,
    values: command.split(' '),
  }).then(res => {
    success({
      type: 'normal',
      content: res,
    })
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
