<script setup lang="ts">
import { useTabs } from '@/store/tabs'
import { TreeNode } from '@/types/redis'
import { useMitt } from '@/useMitt'
import { clipboard, invoke } from '@tauri-apps/api'
import type { ElTree } from 'element-plus'
import Node from 'element-plus/es/components/tree/src/model/node'
import { TreeNodeData } from 'element-plus/es/components/tree/src/tree.type'
import { useConfig } from './useConfig'

const tabsState = useTabs()
const mitt = useMitt()
const configOps = useConfig()
const treeKeys = computed(() =>configOps!.treeKeys.value)
const treeRef = ref<InstanceType<typeof ElTree>|null>(null)
const id = computed(() => configOps!.config.id)
const db = computed(() => unref(configOps!.db))
const isCurrent = (otherId: string) => unref(id) === otherId

mitt.on('searchKeyTree', ({ id, query }) => isCurrent(id) && treeRef.value?.filter(query))

onUnmounted(() => {
  mitt.off('searchKeyTree')
})

const filterNode = (value: string, data: TreeNode | TreeNodeData, node: Node) => {
  const rawData = data as TreeNode
  if (node.isLeaf) {
    return rawData.value.includes(value)
  }
  return rawData.label.includes(value)
}

const rendIcon = () => h('i', { class: 'bi:caret-right-fill w20px h20px' })

const handleNodeClick = (data: TreeNode, node: any) => {
  const isLeaf = !data.children

  if (!isLeaf) {
    return
  }

  const key = `${unref(id)}-${unref(db)}-${data.value}`

  tabsState.addTab({
    id: unref(id),
    db: unref(db),
    type: 'detail',
    key,
    value: data.value,
    name: configOps!.config.name,
    label: `${data.value} | ${configOps?.config.name} | DB${unref(db)}`,
    icon: 'fxemoji:key',
  })
}

interface ContextmenuProps {
  event: MouseEvent
  data: TreeNode
  node: Node
}

const contextmenuRef = ref<HTMLDivElement|null>(null)
const showContextmenu = ref(false)
const contextmenuData = ref<ContextmenuProps>()
onClickOutside(contextmenuRef, () => {
  showContextmenu.value = false
})

const hideMenusOnClick = (e: MouseEvent) => {
  showContextmenu.value = false
}
const HideMenusOnEsc = (ev: KeyboardEvent) => {
  if (ev.key === 'Escape') {
    showContextmenu.value = false
  }
}


const handleContextmenu = (event: MouseEvent, data: TreeNode, node: Node) => {
  showContextmenu.value = true
  contextmenuData.value = { event, data, node }

  nextTick(() => {
    // ??????contextmenuRef??????????????????
    let top = event.clientY
    if (document.body.clientHeight - top < contextmenuRef.value!.clientHeight) {
      top -= contextmenuRef.value!.clientHeight
    }

    contextmenuRef.value!.style.left = `${event.clientX}px`
    contextmenuRef.value!.style.top = `${top}px`

    document.addEventListener('click', hideMenusOnClick, { once: true })
    document.addEventListener('keyup', HideMenusOnEsc, { once: true })
  })
}

const handleDeleteKey = async () => {
  try {
    await invoke('del_key', {
      id: unref(id),
      db: unref(db),
      key: contextmenuData.value?.data.value,
    })

    ElMessage.success(`?????????: ${contextmenuData.value?.data.value}??????`)
    // ????????????????????????????????????
    tabsState.removeTab(
      `${unref(id)}-${unref(db)}-${contextmenuData.value?.data.value}`
    )

    mitt.emit('refresh', { id: unref(id), db: unref(db) })
  } catch (error) {
    ElMessage.error(error as string)
  }
}

const handleDeleteFolder = async () => {
  try {
    await invoke('del_match_keys', {
      id: unref(id),
      db: unref(db),
      matchKey: `${contextmenuData.value?.data.value}*`,
    })

    ElMessage.success(`?????????: ${contextmenuData.value?.data.value}??????`)
    // ????????????????????????????????????????????????????????????
    tabsState.removeTab(
      `${unref(id)}-${unref(db)}-${contextmenuData.value?.data.value}`
    )

    mitt.emit('refresh', { id: unref(id), db: unref(db) })
  } catch (error) {
    ElMessage.error(error as string)
  }
}

const handleCopyKey = () => {
  if (contextmenuData.value) {
    clipboard.writeText(contextmenuData.value.data.value)
  }
}

const handleCommand = (command: string) => {
  switch (command) {
  case 'delete-key':
    handleDeleteKey()
    break
  case 'delete-folder':
    handleDeleteFolder()
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
      <ElTree
        ref="treeRef"
        :data="treeKeys"
        style="max-height: calc(100vh - 252px)"
        :icon="rendIcon()"
        :filter-node-method="filterNode"
        @node-click="handleNodeClick"
        @node-contextmenu="handleContextmenu"
      >
        <template #default="{ node, data }">
          <template v-if="data.children">
            <div flex items-center>
              <i v-if="node.expanded" class="vscode-icons:folder-type-redis-opened w20px h20px" />
              <i v-else class="vscode-icons:folder-type-redis w20px h20px" />
              <span ml1>{{ node.label }} ({{ data.children.length }})</span>
            </div>
          </template>

          <template v-else>
            <div flex items-center>
              <i class="fxemoji:key w20px h20px" />
              <span ml1>{{ node.label }}</span>
            </div>
          </template>
        </template>
      </ElTree>
    </el-scrollbar>

    <!-- contextmenu -->
    <div
      v-show="showContextmenu"
      ref="contextmenuRef"
      class="tree-contextmenu-ops"
    >
      <!-- folder -->
      <template v-if="!contextmenuData?.node.isLeaf">
        <div
          class="contextmenu-item"
          flex
          items-center
          space-x2
          justify-start
          @click="handleCommand('copy-key')"
        >
          <i class="material-symbols:content-copy-outline" />
          <span>
            ??????
          </span>
        </div>
        <div
          class="contextmenu-item"
          flex
          items-center
          space-x2
          justify-start
          @click="handleCommand('delete-folder')"
        >
          <i class="material-symbols:content-copy-outline" />
          <span>
            ????????????
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
            ??????
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
            ??????
          </span>
        </div>
      </template>
    </div>
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
  z-index: 99999;
  background-color: var(--el-bg-color);
  border: 1px solid var(--el-border-color);
  padding: 6px 0;
  border-radius: 4px;
}

.contextmenu-item {
  padding: 6px 12px;
  transition: 0.2s;
}

.contextmenu-item:hover {
  cursor: pointer;
  color: var(--el-color-primary);
  background-color: var(--el-color-primary-light-9);
}
</style>
