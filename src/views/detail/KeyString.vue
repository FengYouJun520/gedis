<script setup lang="ts">
import { useMitt } from '@/useMitt'
import { invoke } from '@tauri-apps/api'
import { AddKeyInfo, KeyInfo } from '@/types/redis'

interface StringProps {
  id: string
  db: number
  keyLabel: string
  keyinfo: KeyInfo
}

const props = defineProps<StringProps>()
const mitt = useMitt()
console.log(props.keyinfo)

const addKeyinfo = ref<AddKeyInfo>({
  key: props.keyLabel,
  type: 'string',
  value: props.keyinfo.value,
})

const handleSave = () => {
  ElMessageBox.confirm('确定要保存吗？', {
    type: 'info',
  }).then(async () => {
    try {
      await invoke('set_key', {
        id: props.id,
        db: props.db,
        keyinfo: unref(addKeyinfo),
      })
      mitt.emit('fetchKeyInfo')
    } catch (error) {
      ElMessage.error(error as string)
    }
  })
    .catch(() => {})
}
</script>

<template>
  <div>
    <el-form :model="addKeyinfo">
      <el-form-item prop="value">
        <el-input v-model="addKeyinfo.value" :rows="24" type="textarea" />
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
