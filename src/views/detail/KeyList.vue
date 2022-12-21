<script setup lang="ts">
import { KeyContentDetail, AddKeyInfo } from '@/types/redis'
import { useMitt } from '@/useMitt'
import { invoke } from '@tauri-apps/api'

interface StringProps {
  id: string
  db: number
  keyLabel: string
}

const props = defineProps<StringProps>()
const id = ref(props.id)
const db = ref(props.db)
const key = ref(props.keyLabel)
const mitt = useMitt()

const keyDetail = ref<KeyContentDetail<string[]>>({
  key: unref(key),
  type: 'string',
  label: '',
  size: 0,
  ttl: -1,
  value: [],
})

const fetchKeyDetail = async () => await invoke<KeyContentDetail>('get_key_detail', {
  id: unref(id),
  db: unref(db),
  key: unref(key),
})

onMounted(async () => {
  try {
    const detail = await fetchKeyDetail()
    console.log('list detail')

    console.log(detail.value)

    keyDetail.value = detail
  } catch (error) {
    ElMessage.error(error as string)
  }
})
</script>

<template>
  <div>
    dd
    <div v-for="item in keyDetail.value" :key="item">
      {{ item }}
    </div>
  </div>
</template>

<style lang="css" scoped>
</style>
