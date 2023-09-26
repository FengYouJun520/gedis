<script setup lang="ts">
import { useTabs } from '@/store/tabs'
import { AddKeyInfo, Keyspace, RedisConfig } from '@/types/redis'
import { useMitt } from '@/useMitt'
import { useConfig } from './useConfig'
import { SelectOption } from 'naive-ui'
import { setKey } from '@/apis/key_ops'

const props = defineProps<{
  keyspaces: Keyspace[] | Record<string, Keyspace[]>
  config: RedisConfig
}>()

const message = useMessage()
const tabsState = useTabs()
const mitt = useMitt()
const configOps = useConfig()
const selectDB = defineModel('db', { default: 0, required: true })
const search = ref('')
const id = computed(() => configOps.config.id)
const db = computed(() => unref(configOps.db))
const isCluster = computed(() => props.config.cluster)

const selectOptions = computed<SelectOption[]>(() => {
  if (unref(isCluster)) {
    return [{
      label: `DB0 (${configOps.treeKeys.value.length})`,
      value: 0,
    }]
  }

  const keyspaces = props.keyspaces as Keyspace[]
  return keyspaces.map(keyspace => ({
    label: `DB${keyspace.db} (${keyspace.len})`,
    value: keyspace.db,
  }))
})

const typeOptions: SelectOption[] = [
  {
    label: 'String',
    value: 'string',
  },
  {
    label: 'List',
    value: 'list',
  },
  {
    label: 'Set',
    value: 'set',
  },
  {
    label: 'ZSet',
    value: 'zset',
  },
  {
    label: 'Hash',
    value: 'hash',
  },
  {
    label: 'Stream',
    value: 'stream',
  },
]

const handleChange = (val: number) => {
  mitt.emit('changeDb', { id: unref(id), db: unref(val) })
}

const handleSearchChange = () => {
  mitt.emit('searchKeyTree', { id: unref(id), query: unref(search) })
}

const visibleDialog = ref(false)
const keyModel = ref<AddKeyInfo>({
  key: '',
  type: 'string',
  value: 'new value',
  id: '*',
  field: 'new field',
  score: 0.0,
})

const handleAddDialog = () => {
  visibleDialog.value = true
}

const handleConfirm = async () => {
  if (!keyModel.value.key) {
    message.warning('key不能为空')
    return
  }

  try {
    if (keyModel.value.type === 'stream') {
      keyModel.value.value = '{"New key": "New value"}'
    }
    await setKey(unref(id), unref(db), keyModel.value)

    // 添加新的选项卡并且跳转
    if (configOps) {
      tabsState.addTab({
        id: unref(id),
        db: unref(db),
        type: 'detail',
        cluster: configOps.config.cluster,
        key: `${unref(id)}-${unref(db)}-${keyModel.value.key}`,
        value: keyModel.value.key,
        name: configOps.config.name,
        label: `${keyModel.value.key} | ${configOps.config.name} | DB${unref(db)}`,
      })
    }

    visibleDialog.value = false
    mitt.emit('refresh', { id: unref(id), db: unref(db) })
  } catch (error) {
    message.error(error as string)
  }
}

const handleCloseDialog = () => {
  visibleDialog.value = false
  keyModel.value = {
    key: '',
    type: 'string',
    value: 'new value',
    id: '*',
    field: 'new field',
    score: 0.0,
  }
}
</script>

<template>
  <div flex flex-col gap-y2 px2 my2>
    <div flex items-center gap-x2>
      <n-select
        v-model:value="selectDB"
        placeholder="Select"
        size="large"
        :disabled="config.cluster"
        :options="selectOptions"
        @update:value="handleChange"
      />
      <n-button type="primary" ghost @click="handleAddDialog">
        新增key
      </n-button>
    </div>
    <div>
      <n-input
        v-model:value="search"
        clearable
        placeholder="搜索"
        @update:value="handleSearchChange"
      />
    </div>

    <n-modal
      v-model:show="visibleDialog"
      title="添加键"
      :auto-focus="false"
      preset="dialog"
      @after-leave="handleCloseDialog"

    >
      <n-form :model="keyModel">
        <n-form-item label="键">
          <n-input v-model:value="keyModel.key" placeholder="请输入键" />
        </n-form-item>
        <n-form-item label="类型">
          <n-select v-model:value="keyModel.type" :options="typeOptions" />
        </n-form-item>
      </n-form>

      <template #action>
        <n-button @click="handleCloseDialog">
          取消
        </n-button>
        <n-button type="primary" @click="handleConfirm">
          确定
        </n-button>
      </template>
    </n-modal>
  </div>
</template>

<style lang="css" scoped>
</style>
