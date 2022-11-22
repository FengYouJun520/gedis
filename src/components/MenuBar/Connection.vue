<script setup lang="ts">
import RightOpertions from '@/components/MenuBar/RightOpertions.vue'
import MenuOperation from '@/components/MenuBar/MenuOperation.vue'
import KeyList from '@/components/MenuBar/KeyList.vue'
import { useRedis } from '@/store/redis'
import { useTabs } from '@/store/tabs'
import { invoke } from '@tauri-apps/api'
import { RedisConfig } from '@/types/redis'
import type { ElMenu } from 'element-plus'
import { keysToTree } from '@/util'
import { createTreeKeysContext } from './useTree'

interface ConnectionProps {
  config: RedisConfig
}

const props = defineProps<ConnectionProps>()

const router = useRouter()
const redisState = useRedis()
const tabsState = useTabs()
const treeKeys = ref<string[]>([])
const menuRef = ref<InstanceType<typeof ElMenu>|null>(null)

const loading = ref(false)
const isOpen = ref(false)
const selectDb = ref(0)

const changeDb = (db: number) => {
  selectDb.value = db
}

createTreeKeysContext({
  db: selectDb,
  treeKeys,
  changeDb,
})

const handleConnection = async (config: RedisConfig) => {
  try {
    // 是否连接
    const isConnection = await invoke<boolean>('is_connection', { id: config.id })

    if (!isConnection) {
      loading.value = true
      await invoke('connection', { config })
    }

    // 获取所有key
    treeKeys.value = await fetchTreeKeys(config.id, unref(selectDb))

    isOpen.value = true

    // 添加选项卡信息，该方法已过滤重复
    tabsState.addTab({
      key: config.id,
      name: config.name,
      db: 0,
      path: '/info',
      query: {
        id: config.id,
      },
      icon: 'emojione:rocket',
    })

    // 第一次连接跳转到信息页
    if (!isConnection) {
      router.push({
        path: '/info',
        query: {
          id: config.id,
        },
      })
    }
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

const handleOpen = async (index: string) => {
  const config = redisState.getConfig(index)
  if (config) {
    await handleConnection(config)
  }
}

const handleclose = (index: string) => {
  console.log(`close index: ${index}`)
}

const handleChange = (value: any) => {
  console.log(value)
}


// 获取指定数据库的所有树型key列表
const fetchTreeKeys = async (id: string, db: number): Promise<string[]> => {
  try {
    const keys = await invoke<string[]>('get_keys_by_db', {
      id,
      db,
    })

    return keysToTree(keys)
  } catch (error) {
    ElMessage.error(error as string)
    isOpen.value = false
    return []
  }
}

watch(selectDb, async () => {
  try {
    if (!isOpen) {
      return
    }

    const keys = await fetchTreeKeys(props.config.id, unref(selectDb))
    treeKeys.value = keys
    console.log(unref(keys))
  } catch (error) {
    ElMessage.error(error as string)
    isOpen.value = false
  }
})
</script>

<template>
  <ElMenu
    ref="menuRef"
    :key="config.id"
    ellipsis
    @select="handleChange"
    @open="handleOpen"
    @close="handleclose"
  >
    <el-sub-menu
      :index="config.id"
    >
      <!-- 标题 -->
      <template #title>
        <div flex-1 flex justify-between items-center mr6>
          <span>{{ config.name }}</span>
          <i v-if="loading" class="uiw:loading animate-spin" />
          <RightOpertions v-else :config="config" />
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
