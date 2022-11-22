<script setup lang="ts">
import { useTabs } from '@/store/tabs'
import type { TabsPaneContext } from 'element-plus'

const tabsState = useTabs()

const handleClick = (pane: TabsPaneContext, event: MouseEvent) => {
  tabsState.setActive(pane.paneName?.toString() || '')
}

const handleRemove = (name: string) => {
  tabsState.removeTab(name)
}
</script>

<template>
  <div
    h-full
    w-full
    py4
  >
    <el-tabs
      v-model="tabsState.currentActive"
      type="border-card"
      w-full
      closable
      @tab-click="handleClick"
      @tab-remove="handleRemove"
    >
      <el-tab-pane
        v-for="tabItem in tabsState.tabs"
        :key="tabItem.key"
        :label="tabItem.name"
        :name="tabItem.key"
      >
        <template #label>
          <div inline-flex items-center space-x2>
            <i :class="tabItem.icon " />
            <span>{{ tabItem.name }}</span>
          </div>
        </template>
      </el-tab-pane>
    </el-tabs>
  </div>
</template>

<style lang="css" scoped>
:deep(.el-tabs__content) {
  display: none;
}
</style>
