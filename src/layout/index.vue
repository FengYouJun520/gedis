<script setup lang="ts">
import MenuBar from '@/components/MenuBar/index.vue'
import { useTabs } from '@/store/tabs'
import Header from './Header.vue'

const tabsState = useTabs()
</script>

<template>
  <el-container h-full>
    <MenuBar />
    <el-container direction="vertical">
      <el-header v-if="tabsState.tabs.length">
        <Header />
      </el-header>
      <el-main relative>
        <el-scrollbar view-style="height: 100%">
          <router-view v-slot="{ Component }">
            <keep-alive>
              <component :is="Component" />
            </keep-alive>
          </router-view>
        </el-scrollbar>
      </el-main>
    </el-container>
  </el-container>
</template>

<style lang="css" scoped>
</style>
