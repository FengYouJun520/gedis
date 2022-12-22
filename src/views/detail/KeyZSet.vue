<script setup lang="ts">
import { AddKeyInfo, KeyContentDetail, KeyInfo } from '@/types/redis'
import { clipboard, invoke } from '@tauri-apps/api'
import FormDataView from './FormDataView.vue'

interface ZSetProps {
  id: string
  db: number
  keyLabel: string
  keyinfo: KeyInfo
}

interface ZSetDetail {
  score: number
  member: string
}

const props = defineProps<ZSetProps>()

const id = ref(props.id)
const db = ref(props.db)
const key = ref(props.keyLabel)
const listValue = ref<ZSetDetail[]>([])
const isEdit = ref(false)
const showDialog = ref(false)

const keyDetail = ref<KeyContentDetail<ZSetDetail[]>>({
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
  const detail = await invoke<KeyContentDetail<ZSetDetail[]>>('get_key_detail', {
    id: unref(id),
    db: unref(db),
    key: props.keyinfo.key,
  })

  keyDetail.value = detail
  listValue.value = detail.value.map(value => ({
    score: value.score,
    member: value.member,
  }))
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
  clipboard.writeText(scope.row.member)
}

const deleteValueByKey = (scope: any) => {
  const value = scope.row.member
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
  showDialog.value = true
  addKeyinfo.value = {
    key: unref(key),
    type: props.keyinfo.type,
    score: 0,
    value: '',
  }
}

const editValueClick = (scope: any) => {
  isEdit.value = true
  showDialog.value = true
  addKeyinfo.value.score = scope.row.score
  addKeyinfo.value.value = scope.row.member
}

const handleCancel = () => {
  isEdit.value = false
  showDialog.value = false
}

const handleConfirm = async (keyinfo: AddKeyInfo, valid: boolean) => {
  try {
    if (!valid || !keyinfo.value) {
      isEdit.value = false
      showDialog.value = false
      return
    }

    // 删除原来值
    if (unref(isEdit)) {
      await invoke('del_key_by_value', {
        id: unref(id),
        db: unref(db),
        key: unref(key),
        value: unref(addKeyinfo).value,
      })
    }

    // 添加新值
    await invoke('set_key', {
      id: unref(id),
      db: unref(db),
      keyinfo,
    })

    await fetchKeyDetail()

    isEdit.value = false
    showDialog.value = false
  } catch (error) {
    ElMessage.error(error as string)
    isEdit.value = false
    showDialog.value = false
  }
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
      <el-table-column prop="score" label="Score" sortable />
      <el-table-column prop="member" label="Member" sortable />
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
              <el-button text @click="editValueClick(scope)">
                <template #icon>
                  <span>
                    <i class="ant-design:edit-outlined" />
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
      :is-edit="isEdit"
      :title="isEdit ? '修改行' : '添加新行'"
      @cancel="handleCancel"
      @confirm="handleConfirm"
    />
  </div>
</template>

<style lang="css" scoped>
</style>
