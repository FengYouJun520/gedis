<script setup lang="ts">
import { useTabs } from '@/store/tabs'
import type { TabsPaneContext } from 'element-plus'

const tabsState = useTabs()
const router = useRouter()
const route = useRoute()

const handleClick = (pane: TabsPaneContext, _event: MouseEvent) => {
  const tabItem = tabsState.getTab(pane.paneName?.toString() || '')
  tabsState.setActive(pane.paneName?.toString() || '')

  router.push({
    path: tabItem?.path,
    query: tabItem?.query,
  })
}

const handleRemove = (name: string) => {
  const tabItem = tabsState.getTab(name)
  tabsState.removeTab(name)

  if (!tabsState.exist) {
    router.push('/')
  } else {
    router.push({
      path: tabItem?.path,
      query: tabItem?.query,
    })
  }
}

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
              <i :class="tabItem.icon" />
              <span>{{ tabItem.name }}</span>
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
    </el-tabs>
  </div>
</template>

<style lang="css" scoped>
:deep(.el-tabs__content) {
  display: none;
}

.tab--active {
  color: var(--el-color-primary);
}
</style>
