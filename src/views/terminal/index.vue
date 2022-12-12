<script setup lang="ts">
import { TabsProps, useTabs } from '@/store/tabs'
import { invoke } from '@tauri-apps/api'
import type { ElInput, ElScrollbar } from 'element-plus'
import { v4 } from 'uuid'

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

const argsRegex = /[\s*]|"(.*)"/
const tabsState = useTabs()

const content = ref('')
const messages = ref<Message[]>([{
  type: 'success',
  content: `${props.tabItem.name} connected!`,
}])

const clearContent = () => {
  content.value = ''
}


const onExecCmd = async () => {
  switch (unref(content)) {
  case 'exit':
    await invoke('dis_connection', { id: props.tabItem.id })
    tabsState.removeTab(`${props.tabItem.id}-${props.tabItem.db}`)
    clearContent()
    nextTick(() => {
      console.log(scrollRef.value)
      scrollRef.value?.scrollTo({ top: divRef.value?.scrollHeight })
    })
    return
  case 'clear':
    messages.value = []
    clearContent()
    nextTick(() => {
      console.log(scrollRef.value)
      scrollRef.value?.scrollTo({ top: divRef.value?.scrollHeight })
    })
    return
  default:
    break
  }

  const args = content.value.split(argsRegex).filter(arg => arg && arg !== '')
  messages.value.push({ type: 'normal', content: `>> ${content.value}` })
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

    nextTick(() => {
      console.log(scrollRef.value)
      scrollRef.value?.scrollTo({ top: divRef.value?.scrollHeight })
    })
  })
    .catch(error => {
      messages.value.push({ type: 'error', content: error as string })
      nextTick(() => {
        console.log(scrollRef.value)
        scrollRef.value?.scrollTo({ top: divRef.value?.scrollHeight })
      })
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
      <ElInput v-model="content" autofocus @keydown.enter="onExecCmd" />
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
</style>
