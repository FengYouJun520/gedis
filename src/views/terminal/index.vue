<script setup lang="ts">
import { RedisConfig } from '@/types/redis'
import { Terminal } from 'xterm'
import { FitAddon } from 'xterm-addon-fit'
import { AttachAddon } from 'xterm-addon-attach'

const route = useRoute()
console.log(JSON.parse(route.query.config as string))
const config = JSON.parse(route.query.config as string) as RedisConfig

const terminalRef = ref<HTMLDivElement|null>(null)
const fitAddon = new FitAddon()
const websocket = useWebSocket('ws://127.0.0.1:8848', {
  onError: (ws, event) => {
    ElMessageBox.confirm('websocket连接失败，请检查网络设置', {
      type: 'error',
      title: 'websocket连接',
      confirmButtonText: '确定',
      showCancelButton: false,
    })
  },
})
const attachAddon = new AttachAddon(websocket.ws.value!, {})

const term = new Terminal({
  tabStopWidth: 4,
  cursorBlink: true,
  cursorStyle: 'block',
  theme: {
    background: '#141414',
  },
  windowOptions: {
  },
})

onMounted(() => {
  if (terminalRef.value) {
    term.loadAddon(fitAddon)
    term.loadAddon(attachAddon)
    term.open(terminalRef.value)
    term.focus()
    fitAddon.fit()
    term.write(`\x1B[1;3;31m${config.name}\x1B[0m $ `)
  }
})

onUnmounted(() => {
  term.dispose()
})
</script>

<template>
  <div ref="terminalRef" />
</template>

<style lang="css" scoped>
</style>
