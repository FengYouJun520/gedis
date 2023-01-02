<script setup lang="ts">
interface ViewerTextProps {
  content: string
  readonly: boolean
}

const props = defineProps<ViewerTextProps>()
const newContent = ref(props.content)

onMounted(() => {
  newContent.value = props.content
})
watch(() => props.content, value => {
  newContent.value = value
})

defineExpose({
  getContent: () => unref(newContent),
  getRowContent: () => unref(newContent),
})
</script>

<template>
  <div>
    <el-input
      v-model="newContent"
      type="textarea"
      :readonly="readonly"
      :autosize="{ minRows: 18, maxRows: 20 }"
    />
  </div>
</template>

<style lang="css" scoped>
</style>
