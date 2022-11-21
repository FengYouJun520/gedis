<script setup lang="ts">
interface Tree {
  label: string
  children?: Tree[]
}

const handleNodeClick = (data: Tree) => {
  console.log(data)
}

const data: Tree[] = [
  {
    label: 'Level one 1',
    children: [
      {
        label: 'Level two 1-1',
        children: [
          {
            label: 'Level three 1-1-1',
          },
        ],
      },
    ],
  },
  {
    label: 'Level one 2',
    children: [
      {
        label: 'Level two 2-1',
        children: [
          {
            label: 'Level three 2-1-1',
          },
        ],
      },
      {
        label: 'Level two 2-2',
        children: [
          {
            label: 'Level three 2-2-1',
          },
        ],
      },
    ],
  },
  {
    label: 'Level one 3',
    children: [
      {
        label: 'Level two 3-1',
        children: [
          {
            label: 'Level three 3-1-1',
          },
        ],
      },
      {
        label: 'Level two 3-2',
        children: [
          {
            label: 'Level three 3-2-1',
          },
        ],
      },
      {
        label: 'Level two 3-3',
        children: [
          {
            label: 'Level three 3-3-1',
          },
        ],
      },
      {
        label: 'Level two 3-4',
        children: [
          {
            label: 'Level three 3-4-1',
          },
        ],
      },
      {
        label: 'Level two 3-5',
        children: [
          {
            label: 'Level three 3-5-1',
          },
        ],
      },
      {
        label: 'Level two 3-6',
        children: [
          {
            label: 'Level three 3-6-1',
          },
        ],
      },
    ],
  },
]

const defaultProps = {
  children: 'children',
  label: 'label',
}
</script>

<template>
  <div mt4>
    <el-tree
      :data="data"
      :props="defaultProps"
      @node-click="handleNodeClick"
    />
  </div>
</template>

<style lang="css" scoped>
</style>
