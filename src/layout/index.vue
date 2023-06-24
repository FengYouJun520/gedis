<script setup lang="ts">
import MenuBar from '@/components/MenuBar.vue'
import Tabs from './Tabs.vue'
</script>

<template>
  <el-container h-full>
    <menu-bar />
    <el-container>
      <el-main>
        <tabs />
      </el-main>
    </el-container>
  </el-container>
</template>

<style lang="css" scoped>

</style>
