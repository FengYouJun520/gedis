<script setup lang="tsx">
import { AddKeyInfo, KeyContentDetail, KeyInfo } from '@/types/redis'
import { clipboard, invoke } from '@tauri-apps/api'
import FormDataView from './FormDataView.vue'
import { DataTableColumns } from 'naive-ui'

interface StreamProps {
  id: string
  db: number
  keyValue: string
  keyinfo: KeyInfo
}

interface StreamDetail {
  id: string
  value: string
}

const props = defineProps<StreamProps>()

const message = useMessage()
const dialog = useDialog()
const id = ref(props.id)
const db = ref(props.db)
const key = ref(props.keyValue)
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

const columns: DataTableColumns = [
  {
    key: 'index',
    title: () => `ID（Total：${keyDetail.value.size}）`,
    render(rowData, rowIndex) {
      return rowIndex
    },
  },
  {
    key: 'id',
    title: 'ID',
  },
  {
    key: 'value',
    title: 'Value',
    ellipsis: {
      tooltip: true,
    },
  },
  {
    key: 'operation',
    title: 'Operation',
    render(rowData, rowIndex) {
      return (
        <n-space size="small">
          <n-tooltip delay={1000} v-slots={{
            trigger: () => (
              <n-button text size="small" onClick={() => copyValue(rowData)}
                v-slots={{
                  icon: () => (
                    <span>
                      <i class="ant-design:copy-outlined" />
                    </span>
                  ),
                }}
              />
            ),
          }}>
          复制值
          </n-tooltip>
          <n-tooltip delay={1000} v-slots={{
            trigger: () => (
              <n-button text size="small" onClick={() => viewDataClick(rowData)}
                v-slots={{
                  icon: () => (
                    <span>
                      <i class="ant-design:eye-outlined" />
                    </span>
                  ),
                }}
              />
            ),
          }}>
            预览
          </n-tooltip>
          <n-tooltip delay={1000} v-slots={{
            trigger: () => (
              <n-button text size="small" onClick={() => deleteValueByKey(rowData)}
                v-slots={{
                  icon: () => (
                    <span>
                      <i class="ant-design:delete-outlined" />
                    </span>
                  ),
                }}
              />
            ),
          }}>
            删除值
          </n-tooltip>
        </n-space>
      )
    },
  },
]

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
    message.error(error as string)
  }
})

watch(() => props.keyinfo, async () => {
  try {
    await fetchKeyDetail()
  } catch (error) {
    message.error(error as string)
  }
})

const copyValue = (rawData: any) => {
  clipboard.writeText(rawData.value)
}

const deleteValueByKey = (rawData: any) => {
  const value = rawData.id
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
      message.error(error as string)
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

const viewDataClick = (rawData: any) => {
  isEdit.value = true
  readonly.value = true
  showDialog.value = true

  addKeyinfo.value.id = rawData.id
  addKeyinfo.value.value = rawData.value
}

const handleCancel = () => {
  isEdit.value = false
  showDialog.value = false
  readonly.value = false
}

const handleConfirm = async (keyinfo: AddKeyInfo) => {
  try {
    // 删除原来值
    if (unref(isEdit)) {
      isEdit.value = false
      showDialog.value = false
      return
    }

    if (!keyinfo.value || unref(readonly)) {
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
      message.error(error.message)
    } else {
      message.error(error as string)
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
      <n-button type="primary" @click="addValueClick">
        添加新行
      </n-button>
    </div>
    <n-data-table :data="listValue" bordered :columns="columns" />

    <form-data-view
      v-model:show="showDialog"
      :model="addKeyinfo"
      :is-edit="isEdit"
      :title="isEdit ? '修改行' : '添加新行'"
      @cancel="handleCancel"
      @confirm="handleConfirm"
    />
  </div>
</template>

<style lang="css" scoped>
</style>
