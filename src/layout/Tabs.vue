<script setup lang="ts">
import { useTabs } from '@/store/tabs'
import type { TabPaneName, TabsPaneContext } from 'element-plus'
import Home from '@/views/home/index.vue'
import Info from '@/views/info/index.vue'
import Detail from '@/views/detail/index.vue'
import Terminal from '@/views/terminal/index.vue'
import { useThemeVars } from 'naive-ui'

const tabsState = useTabs()
const themeVars = useThemeVars()
const handleClick = (pane: TabsPaneContext, _event: Event) => {
  tabsState.setActive(pane.paneName?.toString() || '')
}

const handleRemove = (name: TabPaneName) => {
  tabsState.removeTab(name.toString())
}

onMounted(async () => {
  window.addEventListener('keydown', event => {
    event.stopPropagation()
    if (event.ctrlKey && event.key === 'w') {
      tabsState.removeTab(tabsState.currentActive)
    }
  })
})

const handleCommand = (key: string, command: string) => {
  switch (command) {
  case 'close':
    tabsState.removeTab(key)
    break
  case 'other':
    tabsState.removeOther(key)
    break
  case 'left':
    tabsState.removeLeft(key)
    break
  case 'right':
    tabsState.removeRight(key)
    break
  default:
    break
  }
}
</script>

<template>
  <div h-full w-full>
    <el-tabs
      v-model="tabsState.currentActive"
      type="border-card"
      w-full
      h-full
      @tab-click="handleClick"
      @tab-remove="handleRemove"
    >
      <n-scrollbar style="height: calc(100vh - 70px);">
        <el-tab-pane
          v-for="tabItem in tabsState.tabs"
          :key="tabItem.key"
          class="px-4"
          :name="tabItem.key"
          :closable="tabItem.type !== 'home'"
        >
          <home v-if="tabItem.type === 'home'" />
          <info
            v-if="tabItem.type === 'info'"
            :tab-item="tabItem"
          />
          <detail
            v-if="tabItem.type === 'detail'"
            :tab-item="tabItem"
          />
          <terminal
            v-if="tabItem.type === 'terminal'"
            :tab-item="tabItem"
          />
          <template #label>
            <el-dropdown
              h-full
              trigger="contextmenu"
              @command="(command: string) => handleCommand(tabItem.key, command)"
            >
              <div
                inline-flex
                items-center
                space-x2
                :class="{ 'tab--active': tabItem.key === tabsState.currentActive }"
              >
                <n-tooltip :delay="500">
                  ctrl+w关闭标签页
                  <template #trigger>
                    <div space-x-2>
                      <i :class="tabItem.icon" />
                      <span>{{ tabItem.label }}</span>
                    </div>
                  </template>
                </n-tooltip>
              </div>
              <template #dropdown>
                <el-dropdown-menu>
                  <el-dropdown-item command="close">
                    <div flex items-center>
                      <i class="mdi:close" />
                      <span>关闭</span>
                    </div>
                  </el-dropdown-item>
                  <el-dropdown-item command="other">
                    <div flex items-center>
                      <i class="mdi:close" />
                      <span>关闭其他选项卡</span>
                    </div>
                  </el-dropdown-item>
                  <el-dropdown-item command="left">
                    <div flex items-center>
                      <i class="mdi:close" />
                      <span>关闭左侧选项卡</span>
                    </div>
                  </el-dropdown-item>
                  <el-dropdown-item command="right">
                    <div flex items-center>
                      <i class="mdi:close" />
                      <span>关闭右侧选项卡</span>
                    </div>
                  </el-dropdown-item>
                </el-dropdown-menu>
              </template>
            </el-dropdown>
          </template>
        </el-tab-pane>
      </n-scrollbar>
    </el-tabs>
  </div>
</template>

<style lang="css" scoped>
.el-tabs {
  @apply flex flex-col;
}
:deep(.el-tabs__content) {
  flex: 1;
}

.el-tab-pane {
  height: 100%;
}
.tab--active {
  color: v-bind("themeVars.primaryColor");
}

:deep(.el-tabs__item) .is-icon-close {
  color: v-bind("themeVars.primaryColor");
}

:deep(.el-tabs__item) .is-icon-close:hover {
  color: v-bind("themeVars.primaryColorHover");
  background-color: v-bind("themeVars.closeColorPressed");
}
</style>
