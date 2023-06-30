<script setup lang="tsx">
import { useTabs } from '@/store/tabs'
import type { TabPaneName, TabsPaneContext } from 'element-plus'
import Home from '@/views/home/index.vue'
import Info from '@/views/info/index.vue'
import Detail from '@/views/detail/index.vue'
import Terminal from '@/views/terminal/index.vue'
import { useThemeVars } from 'naive-ui'
import { DropdownMixedOption } from 'naive-ui/es/dropdown/src/interface'

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

const showDropdown = ref(false)
const xRef = ref(0)
const yRef = ref(0)
const currentKey = ref('')

const handleContextMenu = (e: MouseEvent, key: string) => {
  e.preventDefault()
  showDropdown.value = false
  nextTick().then(() => {
    currentKey.value = key
    showDropdown.value = true
    xRef.value = e.clientX
    yRef.value = e.clientY
  })
}

const onClickoutside = () => {
  showDropdown.value = false
}

const dropdownOps: DropdownMixedOption[] = [
  {
    key: 'close',
    label: '关闭当前选项卡',
    icon: () => <i class="mdi:close" />,
  },
  {
    key: 'other',
    label: '关闭其他选项卡',
    icon: () => <i class="mdi:close" />,
  },
  {
    key: 'left',
    label: '关闭左侧选项卡',
    icon: () => <i class="mdi:close" />,
  },
  {
    key: 'right',
    label: '关闭右侧选项卡',
    icon: () => <i class="mdi:close" />,
  },
]

const handleSelect = (command: string) => {
  showDropdown.value = false
  handleCommand(currentKey.value, command)
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
  <div h-full w-full>
    <el-tabs
      v-model="tabsState.currentActive"
      type="card"
      w-full
      h-full
      @tab-click="handleClick"
      @tab-remove="handleRemove"
    >
      <n-scrollbar style="height: calc(100vh - 56px);">
        <el-tab-pane
          v-for="tabItem in tabsState.tabs"
          :key="tabItem.key"
          px-4
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
            <div
              inline-flex
              items-center
              space-x2
              :class="{ 'tab--active': tabItem.key === tabsState.currentActive }"
              @contextmenu="e => handleContextMenu(e, tabItem.key)"
            >
              <n-tooltip :delay="500">
                按Ctrl+w关闭
                <template #trigger>
                  <div space-x-2>
                    <i :class="tabItem.icon" />
                    <span>{{ tabItem.label }}</span>
                  </div>
                </template>
              </n-tooltip>
            </div>
          </template>
        </el-tab-pane>
      </n-scrollbar>
    </el-tabs>
    <n-dropdown
      inverted
      trigger="manual"
      :show="showDropdown"
      :x="xRef"
      :y="yRef"
      :options="dropdownOps"
      @clickoutside="onClickoutside"
      @select="handleSelect"
    />
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

.tab--active, :deep(.el-tabs__item):hover {
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
