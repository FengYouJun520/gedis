<script setup lang="ts">
import { AddKeyInfo } from '@/types/redis'
import FormatViewer from './FormatViewer.vue'
import { FormInst } from 'naive-ui'
interface FormDataViewProps {
  title: string
  model: AddKeyInfo
  isEdit: boolean
  readonly?: boolean
}

const props = defineProps<FormDataViewProps>()
const emit = defineEmits<{
  cancel: [],
  confirm: [keyinfo: AddKeyInfo]
}>()

const show = defineModel<boolean>('show', { required: true })
const message = useMessage()
const formRef = ref<FormInst | null>(null)
const addKeyinfo = ref<AddKeyInfo>(props.model)
const viewRef = ref<InstanceType<typeof FormatViewer> | null>(null)
const content = ref('')

const handleOpen = () => {
  addKeyinfo.value = { ...props.model }
  content.value = props.model.value
}

const handleCancel = () => {
  emit('cancel')
  content.value = ''
}

const handleConfirm = () => {
  formRef.value?.validate(errors => {
    if (errors) {
      message.error('校验失败')
      return
    }

    addKeyinfo.value.value = viewRef.value!.getRowContent()
    emit('confirm', unref(addKeyinfo))
  })
}
</script>

<template>
  <n-modal
    v-model:show="show"
    :title="title"
    preset="dialog"
    :auto-focus="false"
    style="width: 80%;"
    @after-enter="handleOpen"
    @after-leave="handleCancel"
  >
    <n-form ref="formRef" :model="addKeyinfo" label-placement="top">
      <n-form-item v-if="addKeyinfo.type === 'zset'" label="分数" path="score">
        <n-input-number v-model:value="addKeyinfo.score" />
      </n-form-item>
      <n-form-item v-if="addKeyinfo.type === 'hash'" label="Field" path="field">
        <n-input v-model:value="addKeyinfo.field" />
      </n-form-item>
      <n-form-item v-if="addKeyinfo.type === 'stream'" label="ID" path="id">
        <n-input v-model:value="addKeyinfo.id" :disabled="readonly" />
      </n-form-item>

      <format-viewer
        ref="viewRef"
        v-model="content"
        :readonly="readonly"
        :show-format="addKeyinfo.type === 'stream' ? 'json' : 'text'"
      />
    </n-form>

    <template #action>
      <n-space>
        <n-button tertiary @click="handleCancel">
          取消
        </n-button>
        <n-button type="primary" @click="handleConfirm">
          确定
        </n-button>
      </n-space>
    </template>
  </n-modal>
</template>

<style lang="css" scoped>
</style>
