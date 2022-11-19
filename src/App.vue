<script setup lang="ts">
import { useUiState } from './store/ui'

const uiState = useUiState()

watchEffect(() => {
  let theme = uiState.theme
  if (theme === 'system') {
    const isDark = usePreferredDark()
    if (isDark) {
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
  <router-view />
</template>

<style scoped>
</style>
