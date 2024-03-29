<script setup lang="ts">
import keyOpsApi from '@/apis/key_ops'
import { KeyContentDetail, AddKeyInfo, KeyInfo } from '@/types/redis'
import FormatViewer from './FormatViewer.vue'

interface StringProps {
  id: string
  db: number
  keyValue: string
  keyinfo: KeyInfo
}

const props = defineProps<StringProps>()

const message = useMessage()
const dialog = useDialog()
const id = ref(props.id)
const db = ref(props.db)
const key = ref(props.keyValue)

const keyDetail = ref<KeyContentDetail<string>>({
  key: unref(key),
  type: props.keyinfo.type,
  label: '',
  size: 0,
  ttl: -1,
  value: '',
})

const fetchKeyDetail = async () => {
  const detail = await keyOpsApi.getKeyDetail<string>(unref(id), unref(db), unref(key))
  keyDetail.value = detail
  content.value = detail.value
  rawContent.value = detail.value
}

const rawContent = ref('')
const content = ref('')
const viewRef = ref<InstanceType<typeof FormatViewer> | null>(null)

onMounted(async () => {
  await fetchKeyDetail()
})

// 刷新时重新获取最新数据
watch(() => props.keyinfo, fetchKeyDetail)

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

        await keyOpsApi.setKey(props.id, props.db, keyinfo)
        await fetchKeyDetail()
      } catch (error) {
        message.error(error as string)
      }
    },
    onNegativeClick: () => {
      content.value = rawContent.value
    },
  })
}
</script>

<template>
  <n-form :model="keyDetail" :show-label="false" class="flex flex-col justify-between">
    <n-form-item path="value" flex-1 flex-grow-1 flex-shrink-0>
      <format-viewer
        ref="viewRef"
        v-model="content"
        :redis-key="keyValue"

      />
    </n-form-item>
    <n-form-item>
      <n-button type="primary" @click="handleSave">
        保存
      </n-button>
    </n-form-item>
  </n-form>
</template>

<style lang="css" scoped>
</style>
