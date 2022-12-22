<script setup lang="ts">
import { AddKeyInfo, KeyContentDetail, KeyInfo } from '@/types/redis'
import { clipboard, invoke } from '@tauri-apps/api'
import { k } from '@tauri-apps/api/event-2a9960e7'
import FormDataView from './FormDataView.vue'

interface StreamProps {
  id: string
  db: number
  keyLabel: string
  keyinfo: KeyInfo
}

interface StreamDetail {
  id: string
  value: string
}

const props = defineProps<StreamProps>()

const id = ref(props.id)
const db = ref(props.db)
const key = ref(props.keyLabel)
const listValue = ref<StreamDetail[]>([])
const isEdit = ref(false)
const showDialog = ref(false)
const readonly = ref(false)

const keyDetail = ref<KeyContentDetail<StreamDetail[]>>({
  key: unref(key),
  type: props.keyinfo.type,
  label: '',
  size: 0,
  ttl: -1,
  value: [],
})

const addKeyinfo = ref<AddKeyInfo>({
  key: unref(key),
  type: props.keyinfo.type,
  value: '',
})

const fetchKeyDetail = async () => {
  const detail = await invoke<KeyContentDetail<StreamDetail[]>>('get_key_detail', {
    id: unref(id),
    db: unref(db),
    key: props.keyinfo.key,
  })

  keyDetail.value = detail
  listValue.value = detail.value.map(value => ({
    id: value.id,
    value: value.value,
  }))
}

onMounted(async () => {
  try {
    await fetchKeyDetail()
  } catch (error) {
    ElMessage.error(error as string)
  }
})

watch(() => props.keyinfo, async () => {
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
  const value = scope.row.id
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


const addValueClick = () => {
  isEdit.value = false
  readonly.value = false
  showDialog.value = true
  addKeyinfo.value = {
    key: unref(key),
    type: props.keyinfo.type,
    id: '*',
    value: '',
  }
}

const viewDataClick = (scope: any) => {
  isEdit.value = true
  readonly.value = true
  showDialog.value = true

  addKeyinfo.value.id = scope.row.id
  addKeyinfo.value.value = scope.row.value
}

const handleCancel = () => {
  isEdit.value = false
  showDialog.value = false
  readonly.value = false
}

const handleConfirm = async (keyinfo: AddKeyInfo, valid: boolean) => {
  try {
    // 删除原来值
    if (unref(isEdit)) {
      isEdit.value = false
      showDialog.value = false
      return
    }

    if (!valid || !keyinfo.value || unref(readonly)) {
      ElMessage.error('校验失败')
      return
    }

    const obj: object = JSON.parse(keyinfo.value)
    const value = JSON.stringify(objToString(obj))

    // 添加新值
    await invoke('set_key', {
      id: unref(id),
      db: unref(db),
      keyinfo: {
        ...keyinfo,
        value,
      },
    })

    await fetchKeyDetail()

    isEdit.value = false
    showDialog.value = false
  } catch (error) {
    if (error instanceof SyntaxError) {
      ElMessage.error(error.message)
    } else {
      ElMessage.error(error as string)
    }
  }
}

const objToString = (obj: Record<string, any>) => {
  if (obj === undefined || obj == null) {
    return undefined
  }

  let res: Record<string, string> = {}

  for (const k in obj) {
    if (Array.isArray(obj[k])) {
      res[k] = new Array(obj[k]).join(',')
    } else if (typeof obj[k] === 'object') {
      res[k] = '[object Object]'
    } else if (typeof obj[k] === 'bigint' ||
    typeof obj[k] === 'number' ||
    typeof obj[k] === 'boolean') {
      res[k] = `${obj[k]}`
    } else {
      res[k] = obj[k]
    }
  }


  return res
}
</script>

<template>
  <div flex flex-col gap-y-4>
    <div>
      <el-button type="primary" @click="addValueClick">
        添加新行
      </el-button>
    </div>
    <el-table
      :data="listValue"
      border
      stripe
    >
      <el-table-column type="index" :width="200" :label="`ID（Total: ${keyDetail.size}）`" />
      <el-table-column prop="id" label="ID" sortable />
      <el-table-column prop="value" label="value" sortable />
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
              <el-button text @click="viewDataClick(scope)">
                <template #icon>
                  <span>
                    <i class="ant-design:eye-outlined" />
                  </span>
                </template>
              </el-button>
            </el-tooltip>
            <el-tooltip content="删除值" :show-after="1000">
              <el-button type="danger" text @click="deleteValueByKey(scope)">
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

    <FormDataView
      v-model="showDialog"
      :model="addKeyinfo"
      :readonly="readonly"
      :is-edit="isEdit"
      :title="isEdit ? '修改行' : '添加新行'"
      @cancel="handleCancel"
      @confirm="handleConfirm"
    />
  </div>
</template>

<style lang="css" scoped>
</style>
