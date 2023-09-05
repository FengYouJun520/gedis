<script setup lang="tsx">
import KeyList from '@/components/KeyList.vue'
import { TabsProps, useTabs } from '@/store/tabs'
import { invoke } from '@tauri-apps/api'
import { Keyspace, RedisConfig } from '@/types/redis'
import type { ElMenu } from 'element-plus'
import { keysToTree, parseKeyspaces } from '@/util'
import { createConfigContext } from './useConfig'
import { useMitt } from '@/useMitt'
import RightOperations from './RightOperations.vue'
import MenuOperation from './MenuOperation.vue'
import { useThemeVars } from 'naive-ui'

interface ConnectionProps {
  config: RedisConfig
}

const props = defineProps<ConnectionProps>()
const themeVars = useThemeVars()
const message = useMessage()
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

mitt.on('changeDb', ({ id, db }) => isCurrent(id) && changeDb(db))
mitt.on('fetchInfo', async id => isCurrent(id) && await fetchInfo(id))
mitt.on('fetchTreeKeys', async ({ id, db }) => isCurrent(id) && await fetchTreeKeys(id, db))
mitt.on('refresh', async ({ id, db }) => isCurrent(id) && await refresh(id, db))
mitt.on('disConnection', async id => isCurrent(id) && await handleDisConnection(id))

onUnmounted(() => {
  mitt.off('changeDb')
  mitt.off('fetchInfo')
  mitt.off('fetchTreeKeys')
  mitt.off('refresh')
  mitt.off('disConnection')
})

const refresh = async (id: string, db: number) => {
  await fetchInfo(id)
  await fetchTreeKeys(id, db)
}

onUnmounted(() => {
  ping && clearInterval(ping)
})

const changeDb = (db: number) => {
  if (db === unref(selectDb)) {
    return
  }
  if (props.config.cluster) {
    selectDb.value = 0
    message.warning('当前为集群模式，不允许切换数据库')
    return
  }

  selectDb.value = db
}

watch(selectDb, async db => {
  try {
    if (!unref(connected)) {
      return
    }

    await fetchTreeKeys(props.config.id, db)
  } catch (error) {
    message.error(error as string)
    isOpen.value = false
    connected.value = false
  }
})

watch(connected, newConnected => {
  if (newConnected) {
    handlePing()
  } else {
    ping && clearInterval(ping)
  }
})

const fetchInfo = async (id: string) => {
  try {
    const info = await invoke<Record<string, string>>('get_info', { id })
    keyspaces.value = parseKeyspaces(info)
  } catch (error) {
    message.error(error as string)
  }
}

// 获取指定数据库的所有树型key列表
const fetchTreeKeys = async (id: string, db: number) => {
  try {
    const keys = await invoke<string[]>('get_keys_by_db', { id, db })
    treeKeys.value = keysToTree(keys)
  } catch (error) {
    message.error(error as string)
    isOpen.value = false
    connected.value = false
  }
}

let ping: number|null = null
const pingTime = 30 * 1000
const handlePing = () => {
  ping && clearInterval(ping)
  ping = setInterval(async () => {
    try {
      await invoke('ping', { id: props.config.id })
    } catch (error) {
      message.error(error as string)
      await handleDisConnection(props.config.id)
    }
  }, pingTime)
}

const { isPending, start, stop } = useTimeoutFn(() => {
  if (!unref(connected) && !isPending) {
    stop()
    message.warning('未连接，请检查网络是否正常')
  }
  loading.value = false
}, 10000)
stop()

const handleConnection = async (config: RedisConfig, tabs?: TabsProps) => {
  try {
    // 是否连接
    if (!unref(connected)) {
      loading.value = true
      start()
      await invoke('connection', { config })
      connected.value = true
      stop()
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
    message.error(error as string)
    isOpen.value = false
    connected.value = false
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    menuRef.value?.close(config.id, [config.id])
  } finally {
    loading.value = false
    stop()
  }
}

const handleDisConnection = async (id: string) => {
  try {
    await invoke('dis_connection', { id })
    mitt.emit('clearLogs')
    isOpen.value = false
    connected.value = false
    selectDb.value = 0
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    menuRef.value.close(id, [id])
    tabsState.removeTabById(id)
  } catch (error) {
    message.error(error as string)
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
    message.error(error as string)
  }
}

createConfigContext({
  config: props.config,
  db: selectDb,
  treeKeys,
  keyspaces,
  connection: handleConnection,
  disConnection: handleDisConnection,
})
</script>

<template>
  <el-menu
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
        <div w-full flex-1 flex justify-between items-center>
          <n-tooltip :delay="1000">
            {{ config.name }}
            <template #trigger>
              <span
                text-nowrap
                overflow-hidden
                w-full
                text-ellipsis
              >
                {{ config.name }}
              </span>
            </template>
          </n-tooltip>
          <i v-if="loading" class="uiw:loading animate-spin" />
          <right-operations v-else v-model:db="selectDb" :config="config" />
        </div>
      </template>

      <div v-if="isOpen">
        <!-- 操作 -->
        <menu-operation v-model:db="selectDb" :keyspaces="keyspaces" :config="config" />
        <!-- key列表 -->
        <key-list />
      </div>
    </el-sub-menu>
  </el-menu>
</template>

<style lang="css" scoped>
.el-menu {
  border-right: none;
}

:deep(.el-sub-menu__title):hover {
  background-color: v-bind("themeVars.tagColor") ;
}
</style>
