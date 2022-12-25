<script setup lang="ts">
interface ViewerJsonProps {
  content: string
}

const props = defineProps<ViewerJsonProps>()
const newContent = ref(props.content)

onMounted(() => {
  try {
    const obj = JSON.parse(props.content)
    newContent.value = JSON.stringify(obj, null, 4)
  } catch (error) {
    newContent.value = props.content
  }
})


defineExpose({
  getContent: () => unref(newContent),
  getRowContent: () => {
    try {
      const obj = JSON.parse(unref(newContent))
      const content = JSON.stringify(obj, null, 0)
      return content
    } catch (error) {
      return unref(newContent)
    }
  },
})
</script>

<template>
  <div>
    <el-input v-model="newContent" :rows="20" type="textarea" />
  </div>
</template>

<style lang="css" scoped>
</style>
