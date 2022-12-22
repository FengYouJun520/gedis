<script setup lang="ts">
import { AddKeyInfo } from '@/types/redis'
import { clipboard } from '@tauri-apps/api'
import type { ElForm } from 'element-plus'

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
const copyValue = () => {
  clipboard.writeText(unref(addKeyinfo).value)
}

const handleOpen = () => {
  addKeyinfo.value = { ...props.model }
}

const handleCancel = () => {
  emit('cancel')
}

const handleConfirm = () => {
  formRef.value?.validate(valid => {
    if (!valid) {
      ElMessage.error('校验失败')
      emit('confirm', unref(addKeyinfo), valid)
      return
    }

    emit('confirm', unref(addKeyinfo), valid)
  })
}

const streamViewJson = computed(() => {
  const obj:Record<string, any> = JSON.parse(props.model.value)
  return JSON.stringify(obj, null, 2)
})
</script>

<template>
  <el-dialog
    :model-value="modelValue"
    :title="title"
    width="50%"
    append-to-body
    @open="handleOpen"
    @update:model-value="($event) => emit('update:modelValue', $event)"
    @close="handleCancel"
  >
    <ElForm ref="formRef" :model="addKeyinfo">
      <el-form-item v-if="addKeyinfo.type === 'zset'" label="分数" prop="score">
        <el-input-number v-model="addKeyinfo.score" />
      </el-form-item>
      <el-form-item v-if="addKeyinfo.type === 'hash'" label="Field" prop="field">
        <el-input v-model="addKeyinfo.field" />
      </el-form-item>
      <el-form-item v-if="addKeyinfo.type === 'stream'" label="ID" prop="id">
        <el-input v-model="addKeyinfo.id" :disabled="readonly" />
      </el-form-item>

      <el-form-item>
        <el-space>
          <span>Value</span>
          <el-tag size="small">
            Size:&nbsp;{{ addKeyinfo.value.length }}B
          </el-tag>
          <el-button text size="small" @click="copyValue">
            <template #icon>
              <span>
                <i class="ant-design:copy-outlined" />
              </span>
            </template>
          </el-button>
        </el-space>
      </el-form-item>

      <el-form-item v-if="addKeyinfo.type === 'stream' && isEdit">
        <el-input
          v-model="streamViewJson"
          :readonly="readonly"
          type="textarea"
          :rows="8"
        />
      </el-form-item>
      <el-form-item v-else>
        <el-input
          v-model="addKeyinfo.value"
          type="textarea"
          :rows="8"
        />
      </el-form-item>
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
