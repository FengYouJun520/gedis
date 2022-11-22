<script setup lang="ts">
import { useTreeKeys } from './useTree'

interface Tree {
  label: string
  children?: Tree[]
}

const treeKeys = useTreeKeys()

const treeList = computed(() => treeKeys?.treeKeys.value)

const handleNodeClick = (data: Tree) => {
  console.log(data)
}

const defaultProps = {
  children: 'children',
  label: 'label',
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
