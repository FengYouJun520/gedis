<script setup lang="ts">
import MenuBar from '@/components/MenuBar/index.vue'
import Header from './Header.vue'
</script>

<template>
  <el-container h-full>
    <MenuBar />
    <el-container>
      <el-header>
        <Header />
      </el-header>
      <el-main relative>
        <el-scrollbar view-style="height: 100%">
          <router-view />
        </el-scrollbar>
      </el-main>
    </el-container>
  </el-container>
</template>

<style lang="css" scoped>
</style>
