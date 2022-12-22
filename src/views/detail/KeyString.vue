<script setup lang="ts">
import { useMitt } from '@/useMitt'
import { clipboard, invoke } from '@tauri-apps/api'
import { KeyContentDetail, AddKeyInfo, KeyInfo } from '@/types/redis'

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
const mitt = useMitt()

const keyDetail = ref<KeyContentDetail>({
  key: unref(key),
  type: props.keyinfo.type,
  label: '',
  size: 0,
  ttl: -1,
  value: '',
})

const fetchKeyDetail = async () => {
  const detail = await invoke<KeyContentDetail>('get_key_detail', {
    id: unref(id),
    db: unref(db),
    key: unref(key),
  })

  keyDetail.value = detail
}

watch(() => props.keyinfo, async () => {
  try {
    await fetchKeyDetail()
  } catch (error) {
    ElMessage.error(error as string)
  }
})

const handleSave = () => {
  ElMessageBox.confirm('确定要保存吗？', {
    type: 'info',
  }).then(async () => {
    try {
      const keyinfo: AddKeyInfo = {
        key: unref(key),
        type: 'string',
        value: unref(keyDetail).value,
      }

      await invoke('set_key', {
        id: props.id,
        db: props.db,
        keyinfo,
      })

      mitt.emit('fetchKeyInfo')
      await fetchKeyDetail()
    } catch (error) {
      ElMessage.error(error as string)
    }
  })
    .catch(() => {})
}

const copyContent = () => {
  clipboard.writeText(unref(keyDetail).value)
}
</script>

<template>
  <div flex flex-col gap-y-2>
    <el-space>
      <el-tag>
        Size: {{ keyDetail.size }}B
      </el-tag>
      <el-button text size="small" @click="copyContent">
        复制
        <template #icon>
          <span>
            <i class="ant-design:copy-outlined" />
          </span>
        </template>
      </el-button>
    </el-space>
    <el-form :model="keyDetail">
      <el-form-item prop="value">
        <el-input v-model="keyDetail.value" :rows="20" type="textarea" />
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
