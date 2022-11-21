<script setup lang="ts">
import SubMenuTitle from '@/components/MenuBar/SubMenuTitle.vue'
import MenuOperation from '@/components/MenuBar/MenuOperation.vue'
import KeyList from '@/components/MenuBar/KeyList.vue'
import { useRedis } from '@/store/redis'
import { useTabs } from '@/store/tabs'
import { invoke } from '@tauri-apps/api'
import { RedisConfig } from '@/types/redis'
import type { ElMenu } from 'element-plus'

interface ConnectionProps {
  config: RedisConfig
}

defineProps<ConnectionProps>()

const router = useRouter()
const redisState = useRedis()
const tabsState = useTabs()
const menuRef = ref<InstanceType<typeof ElMenu>|null>(null)

const isOpen = ref(false)
const loading = ref(false)

const handleConnection = async (config: RedisConfig) => {
  try {
    loading.value = true
    const isConnection = await invoke<boolean>('is_connection', { id: config.id })
    isOpen.value = isConnection

    if (!isConnection) {
      await invoke('connection', { config })
    }

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
        <SubMenuTitle :config="config" />
      </template>
      <!-- 操作 -->
      <MenuOperation />
      <!-- key列表 -->
      <KeyList />
    </el-sub-menu>
  </ElMenu>
</template>

<style lang="css" scoped>
.el-menu {
  border-right: none;
}
</style>
