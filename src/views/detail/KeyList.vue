<script setup lang="ts">
import { KeyContentDetail, KeyInfo } from '@/types/redis'
import { useMitt } from '@/useMitt'
import { clipboard, invoke } from '@tauri-apps/api'

interface StringProps {
  id: string
  db: number
  keyLabel: string
  keyinfo: KeyInfo
}

const props = defineProps<StringProps>()
const id = ref(props.id)
const db = ref(props.db)
const key = ref(props.keyLabel)
const mitt = useMitt()
const listValue = ref<{value: string}[]>([])

const keyDetail = ref<KeyContentDetail<string[]>>({
  key: unref(key),
  type: 'string',
  label: '',
  size: 0,
  ttl: -1,
  value: [],
})

const fetchKeyDetail = async () => {
  const detail = await invoke<KeyContentDetail<string[]>>('get_key_detail', {
    id: unref(id),
    db: unref(db),
    key: props.keyinfo.key,
  })

  keyDetail.value = detail
  listValue.value = detail.value.map(value => ({ value }))
}

onMounted(async () => {
  try {
    await fetchKeyDetail()
  } catch (error) {
    ElMessage.error(error as string)
  }
})

watch(props, async () => {
  try {
    await fetchKeyDetail()
  } catch (error) {
    ElMessage.error(error as string)
  }
})

const copyValue = (scope: any) => {
  clipboard.writeText(scope.row.value)
}

const deleteValueByKey = (scope: any) => {
  const value = scope.row.value
  ElMessageBox.confirm('你确定要删除该行吗？', {
    type: 'warning',
  }).then(async () => {
    try {
      await invoke('del_key_by_value', {
        id: unref(id),
        db: unref(db),
        key: unref(key),
        value,
      })

      await fetchKeyDetail()
    } catch (error) {
      ElMessage.error(error as string)
    }
  })
    .catch(() => {})
}
</script>

<template>
  <div flex flex-col gap-y-4>
    <div>
      <el-button type="primary">
        添加新行
      </el-button>
    </div>
    <el-table
      :data="listValue"
      border
    >
      <el-table-column type="index" :width="200" :label="`ID（Total: ${keyDetail.size}）`" />
      <el-table-column prop="value" label="Value" />
      <el-table-column label="Operation">
        <template #default="scope">
          <el-space>
            <el-tooltip content="复制值" :show-after="1000">
              <el-button text @click="copyValue(scope)">
                <template #icon>
                  <span>
                    <i class="ant-design:copy-outlined" />
                  </span>
                </template>
              </el-button>
            </el-tooltip>
            <el-tooltip content="编辑值" :show-after="1000">
              <el-button text>
                <template #icon>
                  <span>
                    <i class="ant-design:edit-outlined" />
                  </span>
                </template>
              </el-button>
            </el-tooltip>
            <el-tooltip content="删除值" :show-after="1000">
              <el-button text @click="deleteValueByKey(scope)">
                <template #icon>
                  <span>
                    <i class="ant-design:delete-outlined" />
                  </span>
                </template>
              </el-button>
            </el-tooltip>
          </el-space>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>

<style lang="css" scoped>
</style>
