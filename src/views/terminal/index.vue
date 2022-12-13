<script setup lang="ts">
import { TabsProps, useTabs } from '@/store/tabs'
import { invoke } from '@tauri-apps/api'
import type { ElAutocomplete, ElScrollbar } from 'element-plus'
import { v4 } from 'uuid'
import { Histroy } from './histroy'

interface TerminalProps {
  tabItem: TabsProps
}

interface Message {
  type: 'normal' | 'error' | 'success'
  content: string
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

let histroy = new Histroy()

const addHistroy = (value: string) => {
  histroy.push(value)
}

const changePrev = () => {
  const value = histroy.prevHistroy()
  content.value = value
}

const changeNext = () => {
  const value = histroy.nextHistroy()
  content.value = value
}

const clearContent = () => {
  content.value = ''
}


onMounted(() => {
  nextTick(()=>{
    autoRef.value?.focus()
  })
})

const handleFetchSuggestions = (query: string, cb: Function) => {
  cb([{ value: 'get KEY ...' }])
}
const backBottom = () => {
  nextTick(() => {
    scrollRef.value?.scrollTo({ top: divRef.value?.scrollHeight })
  })
}


const onExecCmd = async () => {
  if (!unref(content)) {
    messages.value.push({ type: 'normal', content: '>> ' })
    backBottom()
    return
  }

  switch (unref(content)) {
  case 'exit':
    await invoke('dis_connection', { id: props.tabItem.id })
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

  const args = content.value.split(argsRegex).filter(arg => arg && arg !== '')
  messages.value.push({ type: 'normal', content: `>> ${content.value}` })
  addHistroy(unref(content))
  clearContent()

  if (args.length > 1 && args[0].toLowerCase() === 'select') {
    db.value = parseInt(args[1])
  }

  invoke<string| any[] | any[][]>('terminal', {
    id: props.tabItem.id,
    db: unref(db),
    args,
  }).then(res => {
    if (typeof res === 'string') {
      messages.value.push({ type: 'normal', content: res })
    } else {
      messages.value.push(res[0])
      for (const item of res[1] as string[]) {
        messages.value.push({ type: 'normal', content: item })
      }
    }
  })
    .catch(error => {
      messages.value.push({ type: 'error', content: error as string })
    })
    .finally(()=>{
      backBottom()
    })
}
</script>

<template>
  <div flex flex-col h-full>
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
        class="auto-complete"
        :fetch-suggestions="handleFetchSuggestions"
        :debounce="10"
        @select="autoRef?.focus()"
        @keydown.enter="onExecCmd"
        @keydown.up="changePrev"
        @keydown.down="changeNext"
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
</style>
