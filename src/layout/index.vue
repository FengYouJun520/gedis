<script setup lang="ts">
import MenuBar from '@/components/MenuBar.vue'
import Tabs from './Tabs.vue'
</script>

<template>
  <el-container h-full>
    <MenuBar />
    <el-container>
      <el-main>
        <Tabs />
      </el-main>
    </el-container>
  </el-container>
</template>

<style lang="css" scoped>

</style>
