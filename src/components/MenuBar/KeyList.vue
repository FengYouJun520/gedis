<script setup lang="ts">
import { useTabs } from '@/store/tabs'
import { useTreeKeys } from './useTree'

interface Tree {
  label: string
  value: string
  children?: Tree[]
}

const router = useRouter()
const tabsState = useTabs()
const treeKeys = useTreeKeys()
const treeList = computed(() => treeKeys?.treeKeys.value)

const handleNodeClick = (data: Tree) => {
  const isLeaf = !data.children

  if (!isLeaf) {
    return
  }

  const path = '/detail'
  const key = `${treeKeys?.config.id}-${treeKeys?.db.value}-${data.value}`
  const query = {
    id: treeKeys?.config.id,
    db: treeKeys?.db.value,
    key: data.value,
  }

  router.push({
    path,
    query,
  })

  tabsState.addTab({
    db: treeKeys?.db.value || 0,
    key,
    name: `${data.value} | ${treeKeys?.config.name} | DB${treeKeys?.db.value}`,
    path,
    query,
    icon: 'emojione-monotone:key',
  })
}
</script>

<template>
  <div mt4>
    <el-scrollbar>
      <el-tree
        :data="treeList"
        style="max-height: 400px"
        @node-click="handleNodeClick"
      />
    </el-scrollbar>
  </div>
</template>

<style lang="css" scoped>
</style>
