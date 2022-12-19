<script setup lang="ts">
import { TabsProps, useTabs } from '@/store/tabs'
import { KeyInfo } from '@/types/redis'
import { useMitt } from '@/useMitt'
import { invoke } from '@tauri-apps/api'

interface DetailProps {
  tabItem: TabsProps
}

const props = defineProps<DetailProps>()
const tabsState = useTabs()
const mitt = useMitt()
const id = computed(() => props.tabItem.id)
const db = computed(() => props.tabItem.db)
const key = ref(props.tabItem.value)
const keyinfo = ref<KeyInfo>({
  key: '',
  label: '',
  total: 0,
  ttl: -1,
  type: '',
  value: '',
})

const fetchKeyInfo = async () => {
  try {
    const res = await invoke<KeyInfo>('get_key_info', {
      id: unref(id),
      db: unref(db),
      key: unref(key),
    })

    keyinfo.value = res
    key.value = res.key
  } catch (error) {
    ElMessage.error(error as string)
  }
}

const handleTTL = async (ttl: number) => {
  try {
    await invoke('set_key_ttl', {
      id: unref(id),
      db: unref(db),
      key: keyinfo.value.key,
      ttl,
    })

    await fetchKeyInfo()
  } catch (error) {
    ElMessage.error(error as string)
  }
}

const handleSaveKey = () => {
  ElMessageBox.confirm('是否保存该键？', {
    type: 'info',
  }).then(async () => {
    await invoke('rename_key', {
      id: unref(id),
      db: unref(db),
      key: unref(key),
      newKey: unref(keyinfo).key,
    })

    mitt.emit('fetchTreeKeys', {
      id: unref(id),
      db: unref(db),
    })

    const tabKey = `${unref(id)}-${unref(db)}-${unref(key)}`

    key.value = unref(keyinfo).key
    const newTabKey = `${unref(id)}-${unref(db)}-${unref(key)}`
    tabsState.editTab(tabKey, {
      ...props.tabItem,
      key: newTabKey,
      label: `${unref(key)} | ${props.tabItem.name} | DB${unref(db)}`,
      value: unref(keyinfo).key,
    })
    tabsState.setActive(newTabKey)
  })
    .catch(() =>{})
}

const handlePersist = () => {
  ElMessageBox.confirm('是否持久化该键？', {
    type: 'info',
  }).then(async () => {
    await handleTTL(-1)
  })
    .catch(()=>{})
}

onMounted(async () => {
  await fetchKeyInfo()
})
</script>

<template>
  <div flex flex-col>
    <el-form :model="keyinfo" inline flex>
      <el-form-item flex-1>
        <el-input v-model="keyinfo.key">
          <template #prepend>
            <span>{{ keyinfo.label }}</span>
          </template>
          <template #suffix>
            <el-tooltip content="保存键" :show-after="1000">
              <i class="ant-design:save-outlined cursor-pointer" @click="handleSaveKey" />
            </el-tooltip>
          </template>
        </el-input>
      </el-form-item>
      <el-form-item flex-1>
        <el-input v-model.number="keyinfo.ttl">
          <template #prepend>
            <span>TTL</span>
          </template>
          <template #suffix>
            <el-space>
              <el-tooltip content="持久化" :show-after="1000">
                <i class="mdi:timer-lock-outline cursor-pointer" @click="handlePersist" />
              </el-tooltip>
              <el-tooltip content="修改过期时间" :show-after="1000">
                <i class="mdi:av-timer cursor-pointer" @click="handleTTL(keyinfo.ttl)" />
              </el-tooltip>
            </el-space>
          </template>
        </el-input>
      </el-form-item>

      <el-form-item>
        <el-space>
          <el-tooltip content="删除键" :show-after="1000">
            <el-button type="danger">
              <template #icon>
                <i class="mdi:delete" />
              </template>
            </el-button>
          </el-tooltip>
          <el-tooltip content="刷新" :show-after="1000">
            <el-button type="success" @click="fetchKeyInfo">
              <template #icon>
                <i class="mdi:refresh" />
              </template>
            </el-button>
          </el-tooltip>
          <el-tooltip content="复制为命令" :show-after="1000">
            <el-button type="primary">
              <template #icon>
                <i class="ant-design:copy-outlined" />
              </template>
            </el-button>
          </el-tooltip>
        </el-space>
      </el-form-item>
    </el-form>

    <div>
      {{ keyinfo.value }}
    </div>
  </div>
</template>

<style lang="css" scoped>
</style>
