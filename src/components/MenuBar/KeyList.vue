<script setup lang="ts">
import { useTabs } from '@/store/tabs'
import Node from 'element-plus/es/components/tree/src/model/node'
import { useConfig } from './useConfig'

interface Tree {
  label: string
  value: string
  children?: Tree[]
}

const router = useRouter()
const tabsState = useTabs()
const configOps = useConfig()
const treeKeys = computed(() => configOps?.treeKeys.value)

const rendIcon = () => h('i', { class: 'bi:caret-right-fill w20px h20px' })

const handleNodeClick = (data: Tree, node: any) => {
  const isLeaf = !data.children

  if (!isLeaf) {
    return
  }

  const path = '/detail'
  const key = `${configOps?.config.id}-${configOps?.db.value}-${data.value}`
  const query = {
    id: configOps?.config.id,
    db: configOps?.db.value,
    key: data.value,
  }

  tabsState.addTab({
    id: configOps!.config.id,
    db: configOps?.db.value || 0,
    key,
    name: `${data.value} | ${configOps?.config.name} | DB${configOps?.db.value}`,
    path,
    query,
    icon: 'emojione-monotone:key',
  })

  router.push({
    path,
    query,
  })
}

interface ContextmenuProps {
  event: MouseEvent
  data: Tree
  node: Node
}

const contextmenuRef = ref<HTMLDivElement|null>(null)
const showContextmenu = ref(false)
const contextmenuData = ref<ContextmenuProps>()
onClickOutside(contextmenuRef, () => {
  showContextmenu.value = false
})

const handleContextmenu = (event: MouseEvent, data: Tree, node: Node) => {
  showContextmenu.value = true
  contextmenuRef.value!.style.left = `${event.clientX}px`
  contextmenuRef.value!.style.top = `${event.clientY}px`
  contextmenuData.value = { event, data, node }
}

const handleAddKey = () => {
  console.log('add-key')
}

const handleDeleteKey = () => {
  console.log('delete-key')
}

const handleCopyKey = () => {
  console.log('copy-key')
}

const handleCommand = (command: string) => {
  switch (command) {
  case 'add-key':
    handleAddKey()
    break
  case 'delete-key':
    handleDeleteKey()
    break
  case 'copy-key':
    handleCopyKey()
    break
  default:
    break
  }

  showContextmenu.value = false
}
</script>

<template>
  <div mt4>
    <el-scrollbar>
      <el-tree
        :data="treeKeys"
        style="max-height: 400px;"
        :icon="rendIcon()"
        @node-click="handleNodeClick"
        @node-contextmenu="handleContextmenu"
      >
        <template #default="{ node, data }">
          <template v-if="data.children">
            <div flex items-center>
              <i v-if="node.expanded" class="mdi:folder-open w20px h20px" />
              <i v-else class="mdi:folder w20px h20px" />
              <span ml1>{{ node.label }}</span>
            </div>
          </template>

          <template v-else>
            <div flex items-center>
              <i class="mdi:key-variant w20px h20px" />
              <span ml1>{{ node.label }}</span>
            </div>
          </template>
        </template>
      </el-tree>
      <!-- contextmenu -->
      <div
        v-show="showContextmenu"
        ref="contextmenuRef"
        class="tree-contextmenu-ops"
      >
        <div>
          <!-- folder -->
          <template v-if="!contextmenuData?.node.isLeaf">
            <div
              class="contextmenu-item"
              flex
              items-center
              space-x2
              justify-start
              @click="handleCommand('add-key')"
            >
              <i class="material-symbols:content-copy-outline" />
              <span>
                添加键
              </span>
            </div>
            <div
              class="contextmenu-item"
              flex
              items-center
              space-x2
              justify-start
              @click="handleCommand('delete-key')"
            >
              <i class="material-symbols:delete-outline" />
              <span>
                删除
              </span>
            </div>
          </template>

          <!-- key -->
          <template v-else>
            <div
              class="contextmenu-item"
              flex
              items-center
              space-x2
              @click="handleCommand('copy-key')"
            >
              <i class="material-symbols:content-copy-outline" />
              <span>
                复制
              </span>
            </div>
            <div
              class="contextmenu-item"
              flex
              items-center
              space-x2
              @click="handleCommand('delete-key')"
            >
              <i class="material-symbols:delete-outline" />
              <span>
                删除
              </span>
            </div>
          </template>
        </div>
      </div>
    </el-scrollbar>
  </div>
</template>

<style lang="css" scoped>
:deep(.el-tree-node__label) {
  flex: 1;
}

:deep(.el-tree-node__expand-icon) {
  margin-right: 0;
  padding: 0;
}

.tree-contextmenu-ops {
  position: fixed;
  transition: .2s;
  z-index: 999;
  background-color: var(--el-bg-color);
  border: 1px solid var(--el-border-color);
  padding: 12px 0;
}

.contextmenu-item {
  padding: 6px 12px;
}

.contextmenu-item:hover {
  cursor: pointer;
  color: var(--el-color-primary);
  background-color: var(--el-color-primary-light-9);
}
</style>
