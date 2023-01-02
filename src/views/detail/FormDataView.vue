<script setup lang="ts">
import { AddKeyInfo } from '@/types/redis'
import type { ElForm } from 'element-plus'
import FormatViewer from './FormatViewer.vue'

interface FormDataViewProps {
  title: string
  modelValue: boolean
  model: AddKeyInfo
  isEdit: boolean
  readonly?: boolean
}

const props = defineProps<FormDataViewProps>()
const emit = defineEmits<{
(e: 'update:modelValue', show: boolean): void,
(e: 'cancel'): void,
(e: 'confirm', addKeyinfo: AddKeyInfo, valid: boolean): void
}>()

const formRef = ref<InstanceType<typeof ElForm> | null>(null)
const addKeyinfo = ref<AddKeyInfo>({ ...props.model })
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
  formRef.value?.validate(valid => {
    if (!valid) {
      ElMessage.error('校验失败')
      emit('confirm', unref(addKeyinfo), valid)
      return
    }

    addKeyinfo.value.value = viewRef.value!.getRowContent()
    emit('confirm', unref(addKeyinfo), valid)
  })
}
</script>

<template>
  <el-dialog
    :model-value="modelValue"
    :title="title"
    width="80%"
    append-to-body
    align-center
    destroy-on-close
    @open="handleOpen"
    @update:model-value="($event) => emit('update:modelValue', $event)"
    @close="handleCancel"
  >
    <ElForm ref="formRef" :model="addKeyinfo" label-position="top">
      <el-form-item v-if="addKeyinfo.type === 'zset'" label="分数" prop="score">
        <el-input-number v-model="addKeyinfo.score" />
      </el-form-item>
      <el-form-item v-if="addKeyinfo.type === 'hash'" label="Field" prop="field">
        <el-input v-model="addKeyinfo.field" />
      </el-form-item>
      <el-form-item v-if="addKeyinfo.type === 'stream'" label="ID" prop="id">
        <el-input v-model="addKeyinfo.id" :disabled="readonly" />
      </el-form-item>

      <FormatViewer
        ref="viewRef"
        :content="content"
        :readonly="readonly"
        :selected="addKeyinfo.type === 'stream' ? 'json' : 'text'"
      />
    </ElForm>

    <template #footer>
      <span class="dialog-footer">
        <el-button @click="handleCancel">取消</el-button>
        <el-button type="primary" @click="handleConfirm">
          确定
        </el-button>
      </span>
    </template>
  </el-dialog>
</template>

<style lang="css" scoped>
</style>
