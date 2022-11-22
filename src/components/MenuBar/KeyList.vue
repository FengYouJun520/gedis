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

const handleNodeClick = (data: Tree) => {
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
        @node-click="handleNodeClick"
      />
    </el-scrollbar>
  </div>
</template>

<style lang="css" scoped>
</style>
