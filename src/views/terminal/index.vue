<script setup lang="ts">
import { TabsProps, useTabs } from '@/store/tabs'
import { clipboard, invoke, shell } from '@tauri-apps/api'
import { allCommands, CommandType } from './command'
import { useMitt } from '@/useMitt'
import { useThemeVars } from 'naive-ui'
import { Terminal } from 'xterm'
import { FitAddon } from 'xterm-addon-fit'
import { useUiState } from '@/store/ui'

interface TerminalProps {
  tabItem: TabsProps
}

interface Message {
  type: 'normal' | 'error' | 'success'
  content: string
}
const props = defineProps<TerminalProps>()
// 在Linux脚本中以 \x1B[ 开始，中间前部分是样式+内容，以 \x1B[0m 结尾
// 示例 \x1B[1;3;31m 内容 \x1B[0m
// fontCss
// 0;-4;字体样式（0;正常 1;加粗 2;变细 3;斜体 4;下划线）
// bgColor
// 30m-37m字体颜色（30m:黑色 31m:红色 32m:绿色 33m:棕色字 34m:蓝色 35m:洋红色/紫色 36m:蓝绿色/浅蓝色 37m:白色）
// 40m-47m背景颜色（40m:黑色 41m:红色 42m:绿色 43m:棕色字 44m:蓝色 45m:洋红色/紫色 46m:蓝绿色/浅蓝色 47m:白色）
// this.term.write(`\x1B[${fontCss}${bgColor}${txt}\x1B[0m`)

const themeVars = useThemeVars()
const mitt = useMitt()
const id = ref(props.tabItem.id)
const db = ref(props.tabItem.db)
const terminalRef = ref<HTMLDivElement>()
const argsRegex = /[\s*]|"(.*)"/
const tabsState = useTabs()
const uiStore = useUiState()

onUnmounted(() => {
  window.removeEventListener('resize', resize)
  fitAddon.dispose()
  terminal.dispose()
})

const fitAddon = new FitAddon()
const command = ref('')
let terminal: Terminal

const resize = () => {
  fitAddon.fit()
}
window.addEventListener('resize', resize)

onMounted(() => {
  terminal = new Terminal({
    tabStopWidth: 4,
    rows: 30,
    cursorBlink: true,
    cursorStyle: 'bar',
    theme: {
      background: themeVars.value.bodyColor,
      foreground: themeVars.value.textColor1,
      cursor: themeVars.value.textColor1,
    },
    fontSize: 16,
    allowTransparency: true,
  })

  terminal.loadAddon(fitAddon)
  terminal.open(terminalRef.value!)
  terminal.writeln(`\x1b[1;32m连接成功：\x1B[1;31m${props.tabItem.name}\x1B[0m`)
  // 第一次必须防抖才可以调整布局
  useDebounceFn(() => {
    terminal.focus()
    fitAddon.fit()
  }, 100)()

  terminal.attachCustomKeyEventHandler(event => {
    if (event.ctrlKey && event.code === 'KeyC' && event.type === 'keydown') {
      const selection = terminal.getSelection()
      if (selection) {
        clipboard.writeText(selection)
        return false
      }
    }

    if (event.ctrlKey && event.code === 'KeyV' && event.type === 'keydown') {
      clipboard.readText().then(text => {
        if (text) {
          terminal.write(text)
          command.value += text
        }
      })
    }
    return true
  })

  terminal.onData(key => {
    switch (key) {
    case '\r': {
      terminal.writeln('')
      onExecCmd()
      command.value = ''
      break
    }
    case '\u007F': {
      terminal.write('\b \b')
      if (command.value.length > 0) {
        command.value = command.value.slice(0, command.value.length - 1)
      }
      break
    }
    case '\u{16}': {
      break
    }
    default: {
      command.value += key
      if (key.length > 0) {
        terminal.write(key)
      }
      break
    }
    }
  })
})

const onExecCmd = async () => {
  let cmd = command.value.trim()
  if (!unref(cmd)) {
    return
  }

  switch (unref(cmd)) {
  case 'exit':
    command.value = ''
    terminal.clear()
    tabsState.removeTab(`${props.tabItem.id}-${props.tabItem.db}`)
    return
  case 'clear':
    command.value = ''
    terminal.clear()
    return
  default:
    break
  }

  if (unref(cmd).toUpperCase()
    .includes('SELECT')) {
    const values = unref(cmd).split(' ')
    if (values.length === 2 && allCommands[values[0].toUpperCase() as CommandType]) {
      const newDb = parseInt(values[1].trim())
      db.value = newDb
      mitt.emit('changeDb', { id: unref(id), db: newDb })
    }
  }

  if (unref(cmd).toLowerCase()
    .includes('help')) {
    // https://redis.io/commands/${command}/
    const values = unref(cmd).split(' ')
    if (values.length === 2 && allCommands[values[1].toUpperCase() as CommandType]) {
      const url = `https://redis.io/commands/${values[1]}/`
      shell.open(url)
      return
    }
  }

  const args = cmd.split(argsRegex).filter(arg => arg && arg !== '')

  invoke('terminal', {
    id: props.tabItem.id,
    db: unref(db),
    args,
  }).then(res => {
    parseResult(res)
  })
    .catch(error => {
      terminal.writeln(`\x1B[31m${error}\x1B[0m`)
    })
}

const parseResult = (result: any) => {
  if (typeof result === 'object' && Array.isArray(result)) {
    for (const item of result) {
      parseResult(item)
    }
  } else {
    terminal.writeln(`\x1B[33m${result}\x1B[0m`)
  }
}
</script>

<template>
  <div ref="terminalRef" />
</template>

<style lang="css">
.terminal {
  height: calc(100vh - 128px);
}
</style>
