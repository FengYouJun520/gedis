<script setup lang="ts">
import RightOpertions from '@/components/MenuBar/RightOpertions.vue'
import MenuOperation from '@/components/MenuBar/MenuOperation.vue'
import KeyList from '@/components/MenuBar/KeyList.vue'
import { TabsProps, useTabs } from '@/store/tabs'
import { invoke } from '@tauri-apps/api'
import { Keyspace, RedisConfig } from '@/types/redis'
import type { ElMenu } from 'element-plus'
import { keysToTree, parseKeyspaces } from '@/util'
import { createConfigContext } from './useConfig'
import { useMitt } from '@/useMitt'

interface ConnectionProps {
  config: RedisConfig
}

const props = defineProps<ConnectionProps>()

const mitt = useMitt()
const tabsState = useTabs()
const treeKeys = ref<string[]>([])
const keyspaces = ref<Keyspace[]>([])
const menuRef = ref<InstanceType<typeof ElMenu>|null>(null)
const connected = ref(false)
const loading = ref(false)
const isOpen = ref(false)
const selectDb = ref(0)

const isCurrent = (id: string) => props.config.id === id

const changeDb = (db: number) => {
  selectDb.value = db
  mitt.emit('changeDb', db)
}

mitt.on('fetchInfo', id=> {
  if (!isCurrent(id)) {
    return
  }
  fetchInfo(id)
})
mitt.on('fetchTreeKeys', ({ id, db }) => {
  if (!isCurrent(id)) {
    return
  }
  fetchTreeKeys(id, db)
})
mitt.on('disConnection', async id => {
  if (!isCurrent(id)) {
    return
  }
  await handleDisConnection(props.config.id)
})

mitt.on('refresh', async ({ id, db }) => {
  if (!isCurrent(id)) {
    return
  }
  await refresh(id, db)
})

onUnmounted(() => {
  mitt.off('fetchInfo')
  mitt.off('fetchTreeKeys')
  mitt.off('disConnection')
})

const refresh = async (id: string, db: number) => {
  await fetchInfo(id)
  await fetchTreeKeys(id, db)
}

let ping: number|null = null
const pingTime = 60 * 1000
const handlePing = () => {
  ping && clearInterval(ping)
  ping = setInterval(async () => {
    try {
      await invoke('ping', { id: props.config.id })
    } catch (error) {
      ElMessage.error(error as string)
      await handleDisConnection(props.config.id)
    }
  }, pingTime)
}

watch(connected, newConnected => {
  if (newConnected) {
    handlePing()
  } else {
    ping && clearInterval(ping)
    handleDisConnection(props.config.id)
  }
})

const fetchInfo = async (id: string) => {
  try {
    const info = await invoke<Record<string, string>>('get_info', { id })
    keyspaces.value = parseKeyspaces(info)
  } catch (error) {
    ElMessage.error(error as string)
  }
}

const handleConnection = async (config: RedisConfig, tabs?: TabsProps) => {
  try {
    // 是否连接
    if (!unref(connected)) {
      loading.value = true
      await invoke('connection', { config })
      connected.value = true
    }

    await refresh(props.config.id, unref(selectDb))

    isOpen.value = true

    // 添加选项卡信息，该方法已过滤重复
    if (tabs) {
      tabsState.addTab(tabs)
    } else {
      tabsState.addTab({
        id: config.id,
        key: config.id,
        value: config.id,
        name: config.name,
        label: config.name,
        db: 0,
        type: 'info',
        icon: 'emojione:rocket',
      })
    }

    nextTick(() => {
      // eslint-disable-next-line @typescript-eslint/ban-ts-comment
      // @ts-ignore
      menuRef.value?.open?.(config.id, [config.id])
    })
  } catch (error) {
    ElMessage.error(error as string)
    isOpen.value = false
    connected.value = false
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    menuRef.value?.close(config.id, [config.id])
  } finally {
    loading.value = false
  }
}

const handleDisConnection = async (id: string) => {
  try {
    await invoke('dis_connection', { id })
    mitt.emit('clearLogs')
    isOpen.value = false
    connected.value = false
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    menuRef.value.close(id, [id])
  } catch (error) {
    ElMessage.error(error as string)
  } finally {
    loading.value = false
  }
}

const handleOpen = async () => {
  try {
    if (unref(connected)) {
      return
    }

    await handleConnection(props.config)
  } catch (error) {
    ElMessage.error(error as string)
  }
}

// 获取指定数据库的所有树型key列表
const fetchTreeKeys = async (id: string, db: number) => {
  try {
    const keys = await invoke<string[]>('get_keys_by_db', { id, db })
    treeKeys.value = keysToTree(keys)
  } catch (error) {
    ElMessage.error(error as string)
    isOpen.value = false
    connected.value = false
  }
}

watch(selectDb, async db => {
  try {
    if (!unref(isOpen) || !unref(connected)) {
      return
    }

    await fetchTreeKeys(props.config.id, db)
  } catch (error) {
    ElMessage.error(error as string)
    isOpen.value = false
    connected.value = false
  }
})

onUnmounted(() => {
  ping && clearInterval(ping)
})

createConfigContext({
  config: props.config,
  db: selectDb,
  treeKeys,
  keyspaces,
  changeDb,
  refresh,
  fetchInfo,
  fetchTreeKeys,
  connection: handleConnection,
  disConnection: handleDisConnection,
})
</script>

<template>
  <ElMenu
    ref="menuRef"
    :key="config.id"
    ellipsis
    @open="handleOpen"
  >
    <el-sub-menu
      :index="config.id"
    >
      <!-- 标题 -->
      <template #title>
        <div flex-1 flex justify-between items-center>
          <span>{{ config.name }}</span>
          <i v-if="loading" class="uiw:loading animate-spin" />
          <RightOpertions v-else :config="config" :db="selectDb" />
        </div>
      </template>

      <div v-if="isOpen">
        <!-- 操作 -->
        <MenuOperation :keyspaces="keyspaces" />
        <!-- key列表 -->
        <KeyList />
      </div>
    </el-sub-menu>
  </ElMenu>
</template>

<style lang="css" scoped>
.el-menu {
  border-right: none;
}
</style>
