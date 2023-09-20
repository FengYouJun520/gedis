<script setup lang="ts">
interface ViewerTextProps {
  readonly?: boolean
}

defineProps<ViewerTextProps>()
const content = defineModel<string>({ required: true })

defineExpose({
  getContent: () => unref(content),
  getRowContent: () => unref(content),
})
</script>

<template>
  <n-input
    v-model:value="content"
    class="viewer-text"
    type="textarea"
    :readonly="readonly"
  />
</template>

<style lang="css" scoped>
.viewer-text {
  height: calc(100vh - 300px);
}
</style>
