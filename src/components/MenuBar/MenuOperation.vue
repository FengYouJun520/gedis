<script setup lang="ts">
import { AddKeyInfo } from '@/types/redis'
import { invoke } from '@tauri-apps/api'
import { useConfig } from './useConfig'

const treeKeysOps = useConfig()
const selectDB = ref(treeKeysOps?.db || 0)

const createOptions = () => {
  const result = []
  for (let i = 0; i < 16; i++) {
    result.push({
      label: `DB${i}`,
      value: i,
    })
  }

  return result
}

const options = createOptions()

const handleChange = (val: number) => {
  treeKeysOps?.changeDb(val)
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
      id: treeKeysOps?.config.id,
      db: treeKeysOps?.db.value,
      keyinfo: keyModel.value,
    })

    await treeKeysOps?.fetchTreeKeys(treeKeysOps?.config.id, treeKeysOps?.db.value)
    visibleDialog.value = false
  } catch (error) {
    ElMessage.error(error as string)
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
        <el-option
          v-for="item in options"
          :key="item.value"
          :label="item.label"
          :value="item.value"
        />
      </el-select>
      <el-button text bg @click="handleAddDialog">
        新增key
      </el-button>
    </div>
    <div>
      <el-input placeholder="搜索" />
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
        <el-button>取消</el-button>
        <el-button type="primary" @click="handleConfirm">
          确定
        </el-button>
      </template>
    </el-dialog>
  </div>
</template>

<style lang="css" scoped>
</style>
