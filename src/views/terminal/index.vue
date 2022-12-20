<script setup lang="ts">
import { TabsProps, useTabs } from '@/store/tabs'
import { invoke, shell } from '@tauri-apps/api'
import type { ElAutocomplete, ElScrollbar } from 'element-plus'
import { v4 } from 'uuid'
import { History } from './history'
import { allCommands, CommandType } from './command'

interface TerminalProps {
  tabItem: TabsProps
}

interface Message {
  type: 'normal' | 'error' | 'success'
  content: string | number
}

const props = defineProps<TerminalProps>()
const db = ref(props.tabItem.db)
const scrollRef = ref<InstanceType<typeof ElScrollbar> | null>(null)
const divRef = ref<InstanceType<typeof HTMLDivElement> | null>(null)
const autoRef = ref<InstanceType<typeof ElAutocomplete> | null>(null)

const argsRegex = /[\s*]|"(.*)"/
const tabsState = useTabs()

const content = ref('')
const messages = ref<Message[]>([{
  type: 'success',
  content: `${props.tabItem.name} connected!`,
}])
const messageDepth = 2000

const addMessage = (message: string|number | Message) => {
  if (unref(messages).length === messageDepth) {
    messages.value.splice(0, messageDepth / 2)
  }

  if (typeof message === 'string' || typeof message === 'number') {
    messages.value.push({ type: 'normal', content: message })
  } else {
    messages.value.push(message)
  }
}

let history = new History(500)

const addHistroy = (value: string) => {
  history.push(value)
}

const changePrev = () => {
  console.log(autoRef.value?.activated)

  if (autoRef.value?.activated) {
    return
  }

  const value = history.prevHistroy()
  content.value = value
}

const changeNext = () => {
  if (autoRef.value?.activated) {
    return
  }

  const value = history.nextHistroy()
  content.value = value
}

const clearContent = () => {
  content.value = ''
  autoRef.value?.close()
}


onMounted(() => {
  nextTick(()=>{
    autoRef.value?.focus()
  })
})

const handleFetchSuggestions = (query: string, cb: Function) => {
  if (!query.trim()) {
    cb([])
    return
  }

  const result = Object.keys(allCommands)
    .filter(key => key.includes(query.trim().toUpperCase()
      .split(' ')[0]))
    .map(key => ({ value: allCommands[key as CommandType] }))

  cb(result)
}
const backBottom = () => {
  nextTick(() => {
    scrollRef.value?.scrollTo({ top: divRef.value?.scrollHeight })
  })
}


const onExecCmd = async () => {
  if (!unref(content)) {
    addMessage('>> ')
    backBottom()
    return
  }

  switch (unref(content)) {
  case 'exit':
    tabsState.removeTab(`${props.tabItem.id}-${props.tabItem.db}`)
    clearContent()
    backBottom()
    return
  case 'clear':
    messages.value = []
    addHistroy(unref(content))

    clearContent()
    backBottom()
    return
  default:
    break
  }

  if (unref(content).toLowerCase()
    .includes('select')) {
    const values = content.value.split(' ')
    if (values.length === 2 && allCommands[values[1].toUpperCase() as CommandType]) {
      db.value = parseInt(values[1]) || db.value
    }
  }

  if (unref(content).toLowerCase()
    .includes('help')) {
    // https://redis.io/commands/${command}/
    const values = content.value.split(' ')
    if (values.length === 2 && allCommands[values[1].toUpperCase() as CommandType]) {
      const url = `https://redis.io/commands/${values[1]}/`
      shell.open(url)
      addHistroy(unref(content))
      clearContent()
      return
    }
  }

  const args = content.value.split(argsRegex).filter(arg => arg && arg !== '')
  addMessage(`>> ${content.value}`)
  addHistroy(unref(content))
  clearContent()

  if (args.length > 1 && args[0].toLowerCase() === 'select') {
    db.value = parseInt(args[1])
  }

  invoke('terminal', {
    id: props.tabItem.id,
    db: unref(db),
    args,
  }).then(res => {
    parseResult(res)
  })
    .catch(error => {
      addMessage({ type: 'error', content: error as string })
    })
    .finally(()=>{
      backBottom()
    })
}

const parseResult = (result: any) => {
  if (typeof result === 'object' && Array.isArray(result)) {
    for (const item of result) {
      parseResult(item)
    }
  } else {
    addMessage(result)
  }
}
</script>

<template>
  <div flex flex-col class="h-[calc(100vh-128px)]">
    <ElScrollbar ref="scrollRef">
      <div ref="divRef" flex-1 p-4 space-y-2>
        <div v-for="(message, i) in messages" :key="v4() + i" :class="`content--${message.type}`">
          {{ message.content }}
        </div>
      </div>
    </ElScrollbar>
    <div relative>
      <ElAutocomplete
        ref="autoRef"
        v-model="content"
        autocomplete="off"
        :trigger-on-focus="false"
        :fetch-suggestions="handleFetchSuggestions"
        :debounce="10"
        @select="autoRef?.focus()"
        @keyup.enter.stop="onExecCmd"
        @keyup.up.stop="changePrev"
        @keyup.down.stop="changeNext"
      />
    </div>
  </div>
</template>

<style lang="css" scoped>
.content--success {
  color: var(--el-color-success);
}
.content--error {
  color: var(--el-color-danger);
}
:deep(.el-autocomplete) {
  width: 100%;
}
.el-scrollbar__view {
  height: 100%;
}
</style>
