<script setup lang="ts">
import { Terminal } from 'xterm'

const terminalRef = ref<HTMLDivElement|null>(null)
const route = useRoute()

console.log(JSON.parse(route.query.config as string))

const term = new Terminal({

})

onMounted(() => {
  if (terminalRef.value) {
    term.open(terminalRef.value)
    term.write('Hello from \x1B[1;3;31mxterm.js\x1B[0m $ ')
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
