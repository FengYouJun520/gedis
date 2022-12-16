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

interface ConnectionProps {
  config: RedisConfig
}

const props = defineProps<ConnectionProps>()

const tabsState = useTabs()
const treeKeys = ref<string[]>([])
const keyspaces = ref<Keyspace[]>([])
const menuRef = ref<InstanceType<typeof ElMenu>|null>(null)

const loading = ref(false)
const isOpen = ref(false)
const selectDb = ref(0)

const changeDb = (db: number) => {
  selectDb.value = db
}

const handleConnection = async (config: RedisConfig, tabs?: TabsProps) => {
  try {
    // 是否连接
    const isConnection = await invoke<boolean>('is_connection', { id: config.id })

    if (!isConnection) {
      loading.value = true
      await invoke('connection', { config })
    }

    const info = await invoke<Record<string, string>>('get_info', { id: config.id })
    keyspaces.value = parseKeyspaces(info)

    // 获取所有key
    await fetchTreeKeys(config.id, unref(selectDb))

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
        db: 0,
        type: 'info',
        icon: 'emojione:rocket',
      })
    }

    await nextTick(() => {
      // eslint-disable-next-line @typescript-eslint/ban-ts-comment
      // @ts-ignore
      menuRef.value?.open?.(config.id, [config.id])
    })
  } catch (error) {
    ElMessage.error(error as string)
    isOpen.value = false
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
    isOpen.value = false
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
    const isConnection = await invoke<boolean>('is_connection', { id: props.config.id })
    if (isConnection) {
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
  }
}

watch(selectDb, async db => {
  try {
    if (!isOpen) {
      return
    }

    await fetchTreeKeys(props.config.id, db)
  } catch (error) {
    ElMessage.error(error as string)
    isOpen.value = false
  }
})

createConfigContext({
  config: props.config,
  db: selectDb,
  treeKeys,
  keyspaces,
  changeDb,
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
        <MenuOperation />
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
