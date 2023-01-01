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
  <div class="viewer-text">
    <el-input v-model="newContent" :readonly="readonly" :rows="18" type="textarea" />
  </div>
</template>

<style lang="css" scoped>
.viewer-text {
  min-width: 250px;
  min-height: 50px;
  height: 100%;
  width: 100%;
}
</style>
