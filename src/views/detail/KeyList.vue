<script setup lang="tsx">
import { AddKeyInfo, KeyContentDetail, KeyInfo } from '@/types/redis'
import { clipboard } from '@tauri-apps/api'
import keyOpsApi from '@/apis/key_ops'
import FormDataView from './FormDataView.vue'
import { DataTableColumns } from 'naive-ui'

interface ListProps {
  id: string
  db: number
  keyValue: string
  keyinfo: KeyInfo
}

const props = defineProps<ListProps>()
const dialog = useDialog()
const message = useMessage()
const id = computed(() => props.id)
const db = computed(() => props.db)
const key = computed(() => props.keyValue)
const keyType = computed(() => props.keyinfo.type)
const listValue = ref<{value: string}[]>([])
const isEdit = ref(false)
const showDialog = ref(false)

const keyDetail = ref<KeyContentDetail<string[]>>({
  key: unref(key),
  type: unref(keyType),
  label: '',
  size: 0,
  ttl: -1,
  value: [],
})


const addKeyinfo = ref<AddKeyInfo>({
  key: unref(key),
  type: unref(keyType),
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
              <n-button text size="small" onClick={() => editValueClick(rowData)}
                v-slots={{
                  icon: () => (
                    <span>
                      <i class="ant-design:edit-outlined" />
                    </span>
                  ),
                }}
              />
            ),
          }}>
            编辑值
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
  const detail = await keyOpsApi.getKeyDetail<string[]>(unref(id), unref(db), props.keyinfo.key)
  keyDetail.value = detail
  listValue.value = detail.value.map(value => ({ value }))
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
  const value = rawData.value
  dialog.warning({
    title: '删除行',
    content: '你确定要删除该行吗？',
    positiveText: '确定',
    negativeText: '取消',
    onPositiveClick: async () => {
      try {
        await keyOpsApi.delKeyByValue(unref(id), unref(db), unref(key), value)
        await fetchKeyDetail()
      } catch (error) {
        message.error(error as string)
      }
    },
  })
}

const addValueClick = () => {
  isEdit.value = false
  showDialog.value = true
  addKeyinfo.value = {
    key: unref(key),
    type: unref(keyType),
    value: '',
  }
}

const editValueClick = (rawData: any) => {
  isEdit.value = true
  showDialog.value = true
  addKeyinfo.value.value = rawData.value
}

const handleCancel = () => {
  isEdit.value = false
  showDialog.value = false
}

const handleConfirm = async (keyinfo: AddKeyInfo) => {
  try {
    if (!keyinfo.value) {
      return
    }

    // 删除原来值
    if (unref(isEdit)) {
      await keyOpsApi.delKeyByValue(unref(id), unref(db), unref(key), unref(addKeyinfo).value)
    }

    // 添加新值
    await keyOpsApi.setKey(unref(id), unref(db), keyinfo)
    await fetchKeyDetail()

    isEdit.value = false
    showDialog.value = false
  } catch (error) {
    message.error(error as string)
  }
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
