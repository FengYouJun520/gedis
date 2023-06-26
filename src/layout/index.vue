<script setup lang="ts">
import MenuBar from '@/components/MenuBar.vue'
import Tabs from './Tabs.vue'
</script>

<template>
  <n-layout has-sider embedded h-full>
    <menu-bar />
    <n-layout :native-scrollbar="false">
      <tabs />
    </n-layout>
  </n-layout>
</template>

<style lang="css" scoped>

</style>
