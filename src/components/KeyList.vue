<script setup lang="tsx">
import { useTabs } from '@/store/tabs'
import { TreeNode } from '@/types/redis'
import { useMitt } from '@/useMitt'
import { clipboard, invoke } from '@tauri-apps/api'
import Node from 'element-plus/es/components/tree/src/model/node'
import { TreeNodeData } from 'element-plus/es/components/tree/src/tree.type'
import { useConfig } from './useConfig'
import { DropdownOption, TreeOption } from 'naive-ui'
import { RenderSwitcherIcon, TreeNodeProps } from 'naive-ui/es/tree/src/interface'
import { onUpdateExpandedKeys } from 'naive-ui/es/tree/src/Tree'

type TreeOptionExt = TreeOption & {
  value: string
}

const message = useMessage()
const tabsState = useTabs()
const mitt = useMitt()
const configOps = useConfig()
const treeKeys = computed<TreeOptionExt[]>(() => {
  function generateOption(options: any[]) : TreeOptionExt[] {
    return options.map(option => ({
      ...option,
      children: option.isLeaf ? option.children : generateOption(option.children),
      prefix: option.isLeaf
        ? () => <i class="fxemoji:key" w-6 h-6 />
        : () => <i class="vscode-icons:folder-type-redis" w-6 h-6 />,
    } as TreeOptionExt))
  }

  return generateOption(configOps?.treeKeys.value || [])
})

const id = computed(() => configOps!.config.id)
const db = computed(() => unref(configOps!.db))
const isCurrent = (otherId: string) => unref(id) === otherId

mitt.on('searchKeyTree', ({ id, query }) => {
  if (isCurrent(id)) {
    pattern.value = query
  }
})

onUnmounted(() => {
  mitt.off('searchKeyTree')
})

const handleNodeClick = (data: TreeOptionExt) => {
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

const handleDeleteKey = async () => {
  if (!selectedOption.value) {
    return
  }

  try {
    await invoke('del_key', {
      id: unref(id),
      db: unref(db),
      key: selectedOption.value.key,
    })

    message.success(`删除键: ${selectedOption.value.value}成功`)
    // 如果有选项卡，删除选项卡
    tabsState.removeTab(
      `${unref(id)}-${unref(db)}-${selectedOption.value.value}`
    )

    mitt.emit('refresh', { id: unref(id), db: unref(db) })
  } catch (error) {
    message.error(error as string)
  }
}

// TODO: 删除失败bug
const handleDeleteFolder = async () => {
  if (!selectedOption.value) {
    return
  }

  try {
    console.log(selectedOption.value)

    await invoke('del_match_keys', {
      id: unref(id),
      db: unref(db),
      matchKey: `${selectedOption.value.value}*`,
    })

    message.success(`删除键: ${selectedOption.value.value}成功`)
    // 如果有选项卡，删除目录下所有相关的选项卡
    tabsState.removeTab(
      `${unref(id)}-${unref(db)}-${selectedOption.value.value}`
    )

    mitt.emit('refresh', { id: unref(id), db: unref(db) })
  } catch (error) {
    message.error(error as string)
  }
}

const handleCopyKey = () => {
  if (selectedOption.value) {
    clipboard.writeText(selectedOption.value.value)
  }
}

const showDropdown = ref(false)
const pattern = ref('')
const xRef = ref(0)
const yRef = ref(0)
const dropdownOptions = ref<DropdownOption[]>([])
const selectedOption = ref<TreeOptionExt>()

const nodeProps: TreeNodeProps = ({ option }) => ({
  onClick() {
    if (option.isLeaf) {
      message.info(`[Click] ${option.label}`)
      handleNodeClick(option as TreeOptionExt)
    }
  },
  onContextmenu(e: MouseEvent): void {
    e.preventDefault()

    dropdownOptions.value = [{
      label: '复制',
      key: 'copy-key',
      icon: () => <i class="material-symbols:content-copy-outline" />,
    }]
    if (option.isLeaf) {
      dropdownOptions.value.push({
        label: '删除',
        key: 'delete-key',
        icon: () => <i class="material-symbols:delete-outline" />,
      })
    } else {
      dropdownOptions.value.push({
        label: '删除文件',
        key: 'delete-folder',
        icon: () => <i class="material-symbols:content-copy-outline" />,
      })
    }
    showDropdown.value = true
    xRef.value = e.clientX
    yRef.value = e.clientY
    // 保存当前右键选中的节点
    selectedOption.value = option as TreeOptionExt
  },
})

const renderSwitcherIconWithExpaned: RenderSwitcherIcon = ({ expanded }) => (
  <>
    <i class="bi:caret-right-fill" />
  </>
)
const updatePrefixWithExpaned: onUpdateExpandedKeys = (keys, option, meta) => {
  if (!meta.node) {
    return
  }

  if (meta.node.isLeaf) {
    return
  }

  switch (meta.action) {
  case 'collapse':
    meta.node.prefix = () => <i class="vscode-icons:folder-type-redis" w-6 h-6 />
    break
  case 'expand':
    meta.node.prefix = () => <i class="vscode-icons:folder-type-redis-opened" w-6 h-6 />
    break
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
}

const handleSelect = (key: string) => {
  showDropdown.value = false
  console.log(key)
  handleCommand(key)
}

const handleClickoutside = () => {
  showDropdown.value = false
}
</script>

<template>
  <div mt4>
    <n-tree
      block-line
      expand-on-click
      virtual-scroll
      style="height: 350px;"
      :data="treeKeys"
      :pattern="pattern"
      :show-irrelevant-nodes="false"
      :node-props="nodeProps"
      :render-switcher-icon="renderSwitcherIconWithExpaned"
      @update:expanded-keys="updatePrefixWithExpaned"
    />


    <n-dropdown
      trigger="manual"
      placement="bottom-start"
      :options="dropdownOptions"
      :x="xRef"
      :y="yRef"
      :show="showDropdown"
      @select="handleSelect"
      @clickoutside="handleClickoutside"
    />
  </div>
</template>

<style lang="css" scoped>
</style>
