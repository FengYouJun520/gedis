<script setup lang="ts">
import AsideBar from '@/components/AsideBar/index.vue'
</script>

<template>
  <el-container h-full>
    <AsideBar />
    <el-container>
      <el-header>
        header
      </el-header>
      <el-main>
        <router-view />
      </el-main>
    </el-container>
  </el-container>
</template>

<style lang="css" scoped>
</style>
