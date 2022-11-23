<script setup lang="ts">
import { useTabs } from '@/store/tabs'
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
  console.log(node)
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
</script>

<template>
  <div mt4>
    <el-scrollbar>
      <el-tree
        :data="treeKeys"
        style="max-height: 400px"
        :icon="rendIcon()"
        @node-click="handleNodeClick"
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
</style>
