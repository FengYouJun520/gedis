<script setup lang="ts">
import { TabsProps, useTabs } from '@/store/tabs'
import { KeyInfo } from '@/types/redis'
import { useMitt } from '@/useMitt'
import { invoke } from '@tauri-apps/api'
import KeyString from './KeyString.vue'
import KeyList from './KeyList.vue'
import KeySet from './KeySet.vue'
import KeyZSet from './KeyZSet.vue'
import KeyHash from './KeyHash.vue'
import KeyStream from './KeyStream.vue'

interface DetailProps {
  tabItem: TabsProps
}

const props = defineProps<DetailProps>()

const initialData: KeyInfo = {
  key: '',
  label: '',
  ttl: -1,
  // 必须为 ''?
  type: '',
}

const tabsState = useTabs()
const mitt = useMitt()
const id = computed(() => props.tabItem.id)
const db = computed(() => props.tabItem.db)
const key = ref(props.tabItem.value)
const keyinfo = ref<KeyInfo>({ ...initialData })

mitt.on('fetchKeyInfo', async id => {
  if (unref(id) !== id) {
    return
  }
  await fetchKeyInfo()
})

onUnmounted(() => {
  mitt.off('fetchKeyInfo')
})

const fetchKeyInfo = async () => {
  const res = await invoke<KeyInfo>('get_key_info', {
    id: unref(id),
    db: unref(db),
    key: unref(key),
  })

  if (res.ttl === -2) {
    return Promise.reject(`指定的键: ${unref(key)}不存在`)
  }

  keyinfo.value = res
  key.value = res.key
}

onMounted(async () => {
  try {
    await fetchKeyInfo()
  } catch (error) {
    ElMessage.error(error as string)
  }
})

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

    mitt.emit('fetchTreeKeys', { id: unref(id), db: unref(db) })
  })
    .catch(() =>{})
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

const handleSetKeyTTL = () => {
  ElMessageBox.confirm(`是否设置该键的过期时间为: ${unref(keyinfo).ttl}？`, {
    type: 'info',
  }).then(async () => {
    await handleTTL(unref(keyinfo).ttl)
  })
    .catch(() => {})
}

const handlePersistKey = () => {
  ElMessageBox.confirm('是否持久化该键？', {
    type: 'info',
  }).then(async () => {
    await handleTTL(-1)
  })
    .catch(()=>{})
}

const handleDeleteKey = () => {
  ElMessageBox.confirm('是否删除该键？', {
    type: 'error',
  }).then(async () => {
    try {
      await invoke('del_key', { id: unref(id), db: unref(db), key: unref(key) })

      tabsState.removeTab(`${unref(id)}-${unref(db)}-${unref(key)}`)
      mitt.emit('refresh', { id: unref(id), db: unref(db) })
    } catch (error) {
      ElMessage.error(error as string
      )
    }
  })
    .catch(() => {})
}

const handleRefresh = async () => {
  try {
    await fetchKeyInfo()
  } catch (error) {
    // key不存在
    ElMessage.error(error as string)
    tabsState.removeTab(`${unref(id)}-${unref(db)}-${props.tabItem.value}`)
    mitt.emit('refresh', { id: unref(id), db: unref(db) })
  }
}


const components: Record<string, any> = {
  string: KeyString,
  list: KeyList,
  set: KeySet,
  zset: KeyZSet,
  hash: KeyHash,
  stream: KeyStream,
}

const comp = shallowRef()
watch(() => keyinfo.value.type, t => {
  if (components[t]) {
    comp.value = components[t]
  } else {
    ElMessage.error(`键的类型: ${t}是不支持的`)
  }
})
</script>

<template>
  <div class="h-[calc(100vh-109px)]">
    <el-form :model="keyinfo" inline flex grow-0>
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
                <i class="mdi:timer-lock-outline cursor-pointer" @click="handlePersistKey" />
              </el-tooltip>
              <el-tooltip content="修改过期时间" :show-after="1000">
                <i class="mdi:av-timer cursor-pointer" @click="handleSetKeyTTL" />
              </el-tooltip>
            </el-space>
          </template>
        </el-input>
      </el-form-item>

      <el-form-item>
        <el-space>
          <el-tooltip content="删除键" :show-after="1000">
            <el-button type="danger" @click="handleDeleteKey">
              <template #icon>
                <i class="mdi:delete" />
              </template>
            </el-button>
          </el-tooltip>
          <el-tooltip content="刷新" :show-after="1000">
            <el-button type="success" @click="handleRefresh">
              <template #icon>
                <i class="mdi:refresh" />
              </template>
            </el-button>
          </el-tooltip>
        </el-space>
      </el-form-item>
    </el-form>

    <component
      :is="comp && comp"
      :id="id"
      :db="db"
      :key-label="key"
      :keyinfo="keyinfo"
    />
  </div>
</template>

<style lang="css" scoped>
</style>
