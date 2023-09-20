<script setup lang="tsx">
import { useTabs } from '@/store/tabs'
import Home from '@/views/home/index.vue'
import Info from '@/views/info/index.vue'
import Detail from '@/views/detail/index.vue'
import Terminal from '@/views/terminal/index.vue'
import { TabsInst } from 'naive-ui'
import { DropdownMixedOption } from 'naive-ui/es/dropdown/src/interface'

const tabsState = useTabs()
const tabsInstRef = ref<TabsInst | null>(null)

const handleClick = (name: string) => {
  tabsState.setActive(name)
  nextTick().then(() => tabsInstRef.value?.syncBarPosition())
}

const handleRemove = (name: string | number) => {
  tabsState.removeTab(name.toString())
  nextTick().then(() => tabsInstRef.value?.syncBarPosition())
}

onMounted(async () => {
  window.addEventListener('keydown', event => {
    event.stopPropagation()
    if (event.ctrlKey && event.key === 'w') {
      tabsState.removeTab(tabsState.currentActive)
      nextTick().then(() => tabsInstRef.value?.syncBarPosition())
    }
  })
})

const showDropdown = ref(false)
const xRef = ref(0)
const yRef = ref(0)
const currentKey = ref('')

const handleContextMenu = (key: string, e: MouseEvent) => {
  e.preventDefault()
  currentKey.value = key
  showDropdown.value = true

  nextTick().then(() => {
    xRef.value = e.clientX
    yRef.value = e.clientY
  })
}

const onClickOutside = () => {
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
    <n-tabs
      ref="tabsInstRef"
      v-model:value="tabsState.currentActive"
      type="card"
      @update:value="handleClick"
      @close="handleRemove"
    >
      <n-tab-pane
        v-for="tabItem in tabsState.getTabs" :key="tabItem.id"
        :name="tabItem.key"
        :closable="tabItem.key !== 'home'"
        display-directive="show"
      >
        <template #tab>
          <n-tooltip :delay="1000">
            {{ tabItem.value }}
            <template #trigger>
              <div
                inline-flex
                items-center
                @contextmenu="e => handleContextMenu(tabItem.key, e)"
              >
                <div space-x-2>
                  <i :class="tabItem.icon" />
                  <span>
                    {{ tabItem.label.length > 30 ? `${tabItem.label.substring(0, 30)}...` : tabItem.label }}
                  </span>
                </div>
              </div>
            </template>
          </n-tooltip>
        </template>

        <home v-if="tabItem.type === 'home'" />
        <info
          v-if="tabItem.type === 'info'"
          :tab-item="tabItem"
        />
        <n-card embedded :bordered="false">
          <detail
            v-if="tabItem.type === 'detail'"
            :tab-item="tabItem"
          />
          <terminal
            v-if="tabItem.type === 'terminal'"
            :tab-item="tabItem"
          />
        </n-card>
      </n-tab-pane>
    </n-tabs>

    <n-dropdown
      trigger="manual"
      :show="showDropdown"
      :x="xRef"
      :y="yRef"
      :options="dropdownOps"
      @clickoutside="onClickOutside"
      @select="handleSelect"
    />
  </div>
</template>

<style lang="css" scoped>
.n-tabs :deep(.n-tabs-nav) {
  @apply backdrop-blur-4 bg-opacity-90 w-full overflow-hidden sticky top-0 z-10;
}
</style>
