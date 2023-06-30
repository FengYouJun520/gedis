<script setup lang="ts">
import { invoke } from '@tauri-apps/api'
import { KeyContentDetail, AddKeyInfo, KeyInfo } from '@/types/redis'
import FormatViewer from './FormatViewer.vue'

interface StringProps {
  id: string
  db: number
  keyLabel: string
  keyinfo: KeyInfo
}

const props = defineProps<StringProps>()

const message = useMessage()
const dialog = useDialog()
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
  rawContent.value = detail.value
}

const rawContent = ref('')
const content = ref('')
const viewRef = ref<InstanceType<typeof FormatViewer> | null>(null)

onMounted(() => {
  fetchKeyDetail()
})

const handleSave = () => {
  dialog.success({
    title: '保存',
    content: '确定要保存吗？',
    positiveText: '确定',
    negativeText: '取消',
    onPositiveClick: async () => {
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
        message.error(error as string)
      }
    },
    onNegativeClick: () => {
      console.log('取消')
      console.log(toValue(rawContent))

      content.value = rawContent.value
    },
  })
}
</script>

<template>
  <div>
    <el-form :model="keyDetail">
      <el-form-item prop="value">
        <format-viewer
          ref="viewRef"
          :content="content"
          :redis-key="keyLabel"
        />
      </el-form-item>
      <el-form-item>
        <n-button type="primary" @click="handleSave">
          保存
        </n-button>
      </el-form-item>
    </el-form>
  </div>
</template>

<style lang="css" scoped>
</style>
