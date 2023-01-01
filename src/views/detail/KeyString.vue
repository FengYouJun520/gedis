<script setup lang="ts">
import { invoke } from '@tauri-apps/api'
import { KeyContentDetail, AddKeyInfo, KeyInfo } from '@/types/redis'
import FormmatViewer from './FormmatViewer.vue'

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

const keyDetail = ref<KeyContentDetail<string>>({
  key: unref(key),
  type: props.keyinfo.type,
  label: '',
  size: 0,
  ttl: -1,
  value: '',
})

const fetchKeyDetail = async () => {
  const detail = await invoke<KeyContentDetail<string>>('get_key_detail', {
    id: unref(id),
    db: unref(db),
    key: unref(key),
  })

  keyDetail.value = detail
  content.value = detail.value
}

const content = ref('')

onMounted(() => {
  fetchKeyDetail()
})

const handleSave = () => {
  ElMessageBox.confirm('确定要保存吗？', {
    type: 'info',
  }).then(async () => {
    try {
      const content = viewRef.value!.getRowContent()
      if (!content) {
        return
      }

      const keyinfo: AddKeyInfo = {
        key: unref(key),
        type: 'string',
        value: content,
      }

      await invoke('set_key', {
        id: props.id,
        db: props.db,
        keyinfo,
      })

      await fetchKeyDetail()
    } catch (error) {
      ElMessage.error(error as string)
    }
  })
    .catch(() => {})
}

const viewRef = ref<InstanceType<typeof FormmatViewer> | null>(null)
</script>

<template>
  <div flex flex-col gap-y-2>
    <el-form :model="keyDetail">
      <el-form-item prop="value">
        <FormmatViewer
          ref="viewRef"
          :content="content"
          :redis-key="keyLabel"
        />
      </el-form-item>
      <el-form-item>
        <el-button type="primary" @click="handleSave">
          保存
        </el-button>
      </el-form-item>
    </el-form>
  </div>
</template>

<style lang="css" scoped>
</style>
