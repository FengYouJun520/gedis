<script setup lang="ts">
import { useUiState } from './store/ui'
import Layout from '@/layout/index.vue'
import { darkTheme } from 'naive-ui'
import hljs from 'highlight.js/lib/core'
import { allCommands } from './views/terminal/command'
import zhCn from 'element-plus/dist/locale/zh-cn.mjs'
import { useUpdater } from './updater'

useUpdater()

onMounted(() => {
  if (import.meta.env.DEV) {
    return
  }

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
const darkMode = ref(false)
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
    darkMode.value = true
    document.documentElement.className = 'dark'
  } else {
    darkMode.value = false
    document.documentElement.className = ''
  }
})

hljs.registerLanguage('redis-log', () => ({
  case_insensitive: true,
  keywords: Object.keys(allCommands).map(cmd => cmd.toLowerCase()),
  contains: [
  ],
}))
</script>

<template>
  <el-config-provider :locale="zhCn">
    <n-config-provider
      style="height: 100%;"
      :theme="darkMode ? darkTheme : null"
      :hljs="hljs"
    >
      <n-notification-provider>
        <n-dialog-provider>
          <n-message-provider>
            <n-loading-bar-provider>
              <layout />
              <n-global-style />
            </n-loading-bar-provider>
          </n-message-provider>
        </n-dialog-provider>
      </n-notification-provider>
    </n-config-provider>
  </el-config-provider>
</template>

<style scoped>
</style>
