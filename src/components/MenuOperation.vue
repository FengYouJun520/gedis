<script setup lang="ts">
import { useTabs } from '@/store/tabs'
import { AddKeyInfo, Keyspace, RedisConfig } from '@/types/redis'
import { useMitt } from '@/useMitt'
import { invoke } from '@tauri-apps/api'
import { useConfig } from './useConfig'

const props = defineProps<{
  keyspaces: Keyspace[]
  config: RedisConfig
}>()

const message = useMessage()
const tabsState = useTabs()
const mitt = useMitt()
const configOps = useConfig()
const selectDB = defineModel('db', { default: 0, required: true })
const search = ref('')
const id = computed(() => configOps!.config.id)
const db = computed(() => unref(configOps!.db))
const isCluster = computed(() => props.config.cluster)

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
  try {
    if (keyModel.value.type === 'stream') {
      keyModel.value.value = '{"New key": "New value"}'
    }
    await invoke('set_key', {
      id: unref(id),
      db: unref(db),
      keyinfo: keyModel.value,
    })

    // 添加新的选项卡并且跳转
    if (configOps) {
      tabsState.addTab({
        id: unref(id),
        db: unref(db),
        type: 'detail',
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
      <el-select
        v-model="selectDB"
        placeholder="Select"
        size="large"
        @change="handleChange"
      >
        <template v-if="isCluster">
          <el-option
            :label="`DB0 (${configOps?.treeKeys.value.length})`"
            :value="0"
          />
        </template>
        <template v-else>
          <el-option
            v-for="item in props.keyspaces"
            :key="item.db"
            :label="`DB${item.db} (${item.len})`"
            :value="item.db"
          />
        </template>
      </el-select>
      <el-button text bg @click="handleAddDialog">
        新增key
      </el-button>
    </div>
    <div>
      <el-input
        v-model="search"
        clearable
        placeholder="搜索"
        @update:model-value="handleSearchChange"
      />
    </div>

    <el-dialog
      v-model="visibleDialog"
      title="添加键"
      width="50%"
      append-to-body
      destroy-on-close
      @close="handleCloseDialog"

    >
      <el-form
        :model="keyModel"
        label-position="top"
      >
        <el-form-item label="键" required>
          <el-input v-model="keyModel.key" placeholder="键" />
        </el-form-item>
        <el-form-item label="类型" required placeholder="类型">
          <el-select v-model="keyModel.type" w-full>
            <el-option label="String" value="string">
              String
            </el-option>
            <el-option label="List" value="list">
              List
            </el-option>
            <el-option label="Set" value="set">
              Set
            </el-option>
            <el-option label="ZSet" value="zset">
              ZSet
            </el-option>
            <el-option label="Hash" value="hash">
              Hash
            </el-option>
            <el-option label="Stream" value="stream">
              Stream
            </el-option>
          </el-select>
        </el-form-item>
      </el-form>

      <template #footer>
        <el-button @click="handleCloseDialog">
          取消
        </el-button>
        <el-button type="primary" @click="handleConfirm">
          确定
        </el-button>
      </template>
    </el-dialog>
  </div>
</template>

<style lang="css" scoped>
</style>
