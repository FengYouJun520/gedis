<script setup lang="ts">
interface ViewerTextProps {
  content: string
}

const props = defineProps<ViewerTextProps>()
const newContent = ref(props.content)
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
    <el-input v-model="newContent" :rows="20" type="textarea" />
  </div>
</template>

<style lang="css" scoped>
</style>
