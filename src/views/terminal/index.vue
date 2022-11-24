<script setup lang="ts">
import { useTabs } from '@/store/tabs'
import { RedisConfig } from '@/types/redis'
import { EventMessage } from '@/types/vue-web-terminal'
import { invoke } from '@tauri-apps/api'
import Terminal from 'vue-web-terminal'

const route = useRoute()
const router = useRouter()
const tabsState = useTabs()
const db = parseInt(route.query.db as string)
const config = JSON.parse(route.query.config as string) as RedisConfig

onMounted(() => {
  Terminal.$api.focus()
})

const onExecCmd = async (key: string, command: string, success: EventMessage, failed: EventMessage) => {
  if (key === 'exit') {
    await invoke('dis_connection', { id: config.id })
    tabsState.removeTab(`${config.id}-${db}`)
    const tab = tabsState.getTab(tabsState.currentActive)
    router.push({
      path: tab?.path,
      query: tab?.query,
    })
    return
  }


  invoke<string>('terminal', {
    id: config.id,
    db: parseInt(route.query.db as string),
    values: command.split(' '),
  }).then(res => {
    console.log(res)

    success({
      type: 'normal',
      content: res,
    })
  })
    .catch(error => {
      console.log(error)
      success({
        type: 'normal',
        content: error,
      })
    })
}
</script>

<template>
  <Terminal
    :name="config.id"
    :title="config.name"
    :context="config.name"
    @exec-cmd="onExecCmd"
  />
</template>

<style lang="css" scoped>
</style>
