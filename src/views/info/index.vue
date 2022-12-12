<script setup lang="ts">
import { TabsProps } from '@/store/tabs'
import { invoke } from '@tauri-apps/api'

interface InfoProps {
  tabItem: TabsProps
}

const props = defineProps<InfoProps>()
const keyinfo = ref({})


const fetchInfo = async () => {
  const isConnection = await invoke<boolean>('is_connection', { id: props.tabItem.id })
  if (!isConnection) {
    return
  }
  // 连接成功
  // 获取客户端信息
  const info = await invoke<Record<string, string>>('get_info', { id: props.tabItem.id })
}

onMounted(async () => {
  await fetchInfo()
})
</script>

<template>
  <div>
    info
    {{ keyinfo }}
  </div>
</template>

<style lang="css" scoped>
</style>
