<script setup lang="ts">
import { useUiState } from './store/ui'
import Layout from '@/layout/index.vue'
import zhCn from 'element-plus/dist/locale/zh-cn.mjs'

onMounted(() => {
  window.addEventListener('contextmenu', event => {
    event.preventDefault()
  })

  window.addEventListener('keydown', event => {
    if (event.key === 'F5' ||
     event.ctrlKey && event.key === 'r' ||
     event.ctrlKey && event.key === 'F5') {
      event.preventDefault()
    }
  })
})

const uiState = useUiState()

watchEffect(() => {
  let theme = uiState.theme
  if (theme === 'system') {
    const isDark = usePreferredDark()
    if (unref(isDark)) {
      theme = 'dark'
    } else {
      theme = 'light'
    }
  }

  if (theme === 'dark') {
    document.documentElement.className = 'dark'
  } else {
    document.documentElement.className = ''
  }
})
</script>

<template>
  <el-config-provider :locale="zhCn">
    <Layout />
  </el-config-provider>
</template>

<style scoped>
</style>
