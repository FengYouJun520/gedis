<script setup lang="tsx">
import { TabsProps, useTabs } from '@/store/tabs'
import { KeyInfo } from '@/types/redis'
import { useMitt } from '@/useMitt'
import keyOpsApi from '@/apis/key_ops'
import KeyString from './KeyString.vue'
import KeyList from './KeyList.vue'
import KeySet from './KeySet.vue'
import KeyZSet from './KeyZSet.vue'
import KeyHash from './KeyHash.vue'
import KeyStream from './KeyStream.vue'
import { Component as VueComponent } from 'vue'

interface DetailProps {
  tabItem: TabsProps
}

const props = defineProps<DetailProps>()
const dialog = useDialog()
const message = useMessage()
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
  const res = await keyOpsApi.getKeyInfo(unref(id), unref(db), unref(key))

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
    message.error(error as string)
  }
})

const handleSaveKey = () => {
  dialog.success({
    title: '保存键',
    content: '是否保存该键？',
    positiveText: '确定',
    negativeText: '取消',
    onPositiveClick: async () => {
      try {
        await keyOpsApi.renameKey(unref(id), unref(db), unref(key), unref(keyinfo).key)

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
      } catch (error) {
        message.error(error as string)
      }
    },
  })
}


const handleTTL = async (ttl: number) => {
  try {
    await keyOpsApi.setKeyTTL(unref(id), unref(db), keyinfo.value.key, ttl)

    await fetchKeyInfo()
  } catch (error) {
    message.error(error as string)
  }
}

const handleSetKeyTTL = () => {
  dialog.success({
    title: '设置ttl',
    content: `是否设置该键的过期时间为: ${unref(keyinfo).ttl}？`,
    positiveText: '确定',
    negativeText: '取消',
    onPositiveClick: async () => {
      await handleTTL(unref(keyinfo).ttl)
    },
  })
}

const handlePersistKey = () => {
  dialog.success({
    title: '持久化',
    content: '是否持久化该键？',
    positiveText: '确定',
    negativeText: '取消',
    onPositiveClick: async () => {
      await handleTTL(-1)
    },
  })
}

const handleDeleteKey = () => {
  dialog.success({
    title: '删键',
    content: '是否删除该键？',
    positiveText: '确定',
    negativeText: '取消',
    onPositiveClick: async () => {
      try {
        await keyOpsApi.delKey(unref(id), unref(db), unref(key))

        tabsState.removeTab(`${unref(id)}-${unref(db)}-${unref(key)}`)
        mitt.emit('refresh', { id: unref(id), db: unref(db) })

        message.success(() =>
          <span>
            键：
            <n-tag
              type="success"
              size="small"
              bordered={false}
            >
              {unref(key)}
            </n-tag>
            &nbsp;删除成功
          </span>
        )
      } catch (error) {
        message.error(error as string)
      }
    },
  })
}

const handleRefresh = async () => {
  try {
    await fetchKeyInfo()
  } catch (error) {
    // key不存在
    message.error(error as string)
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

const comp = shallowRef<VueComponent>()
watch(() => keyinfo.value.type, t => {
  if (components[t]) {
    comp.value = components[t]
  } else {
    message.error(`键的类型: ${t}是不支持的`)
  }
})
</script>

<template>
  <div class="h-[calc(100vh-96px)]" flex flex-col>
    <n-form
      label
      :show-label="false"
      inline
      :model="keyinfo"
    >
      <n-form-item>
        <n-input-group>
          <n-input-group-label>
            <span>{{ keyinfo.label }}</span>
          </n-input-group-label>
          <n-tooltip :delay="1000" :show-arrow="false">
            {{ keyinfo.key }}
            <template #trigger>
              <n-input v-model:value="keyinfo.key" />
            </template>
          </n-tooltip>
          <n-tooltip :delay="1000">
            保存键
            <template #trigger>
              <n-button type="primary" tertiary @click="handleSaveKey">
                <template #icon>
                  <i class="ant-design:save-outlined cursor-pointer" />
                </template>
              </n-button>
            </template>
          </n-tooltip>
        </n-input-group>
      </n-form-item>
      <n-form-item>
        <n-input-group>
          <n-input-group-label>
            <span>TTL</span>
          </n-input-group-label>
          <n-input-number v-model:value="keyinfo.ttl" />
          <n-tooltip :delay="1000">
            持久化
            <template #trigger>
              <n-button type="primary" tertiary @click="handlePersistKey">
                <template #icon>
                  <i class="mdi:timer-lock-outline cursor-pointer" />
                </template>
              </n-button>
            </template>
          </n-tooltip>
          <n-tooltip :delay="1000">
            修改过期时间
            <template #trigger>
              <n-button type="primary" tertiary @click="handleSetKeyTTL">
                <template #icon>
                  <i class="mdi:av-timer cursor-pointer" />
                </template>
              </n-button>
            </template>
          </n-tooltip>
        </n-input-group>
      </n-form-item>
      <n-form-item>
        <n-space :size="4" :wrap="false">
          <n-button type="error" tertiary @click="handleDeleteKey">
            <template #icon>
              <i class="mdi:delete" />
            </template>
          </n-button>
          <n-button type="primary" tertiary @click="handleRefresh">
            <template #icon>
              <i class="mdi:refresh" />
            </template>
          </n-button>
        </n-space>
      </n-form-item>
    </n-form>

    <component
      :is="comp"
      :id="id"
      :db="db"
      :key-value="key"
      :keyinfo="keyinfo"
      class="flex-1"
    />
  </div>
</template>

<style lang="css" scoped>
</style>
