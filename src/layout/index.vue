<script setup lang="ts">
import MenuBar from '@/components/MenuBar/index.vue'
</script>

<template>
  <el-container h-full>
    <MenuBar />
    <el-container>
      <el-header>
        header
      </el-header>
      <el-main relative>
        <router-view />
      </el-main>
    </el-container>
  </el-container>
</template>

<style lang="css" scoped>
</style>
