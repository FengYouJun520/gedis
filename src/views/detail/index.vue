<script setup lang="ts">
import { TabsProps } from '@/store/tabs'

interface DetailProps {
  tabItem: TabsProps
}

const props = defineProps<DetailProps>()
</script>

<template>
  <div>
    detail
  </div>
</template>

<style lang="css" scoped>
</style>
