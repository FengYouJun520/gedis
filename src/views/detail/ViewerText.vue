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
    type="textarea"
    :readonly="readonly"
    :autosize="{ minRows: 16, maxRows: 20 }"
  />
</template>

<style lang="css" scoped>
</style>
