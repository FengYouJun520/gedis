<script setup lang="ts">
import { invoke } from '@tauri-apps/api'

const route = useRoute()
const keyinfo = ref({})

const id = route.query.id as string

const fetchInfo = async () => {
  const isConnection = await invoke<boolean>('is_connection', { id })
  if (!isConnection) {
    return
  }
  // 连接成功
  // 获取客户端信息
  const info = await invoke<Record<string, string>>('get_info', { id })
  console.log(info)
}

onMounted(async () => {
  await fetchInfo()
})
</script>

<template>
  <div>
    info id: {{ $route.query.id }}
    {{ keyinfo }}
  </div>
</template>

<style lang="css" scoped>
</style>
