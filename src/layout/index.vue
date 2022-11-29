<script setup lang="ts">
import MenuBar from '@/components/MenuBar/index.vue'
import Tabs from './Tabs.vue'
</script>

<template>
  <el-container h-full>
    <MenuBar />
    <el-container direction="vertical">
      <el-main relative>
        <Tabs />
      </el-main>
    </el-container>
  </el-container>
</template>

<style lang="css" scoped>

</style>
