<script setup lang="ts">
import { useUiState } from './store/ui'
import Layout from '@/layout/index.vue'

onMounted(() => {
  window.addEventListener('contextmenu', event => {
    event.preventDefault()
  })
})

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
  <Layout />
</template>

<style scoped>
</style>
