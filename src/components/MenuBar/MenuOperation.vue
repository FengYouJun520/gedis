<script setup lang="ts">
const selectDB = ref(0)

const options = [
  {
    value: 0,
    label: 'DB0',
  },
  {
    value: 1,
    label: 'DB1',
  },
]
</script>

<template>
  <div flex flex-col gap-y2 px2 my2>
    <div flex items-center gap-x2>
      <el-select v-model="selectDB" placeholder="Select" size="large">
        <el-option
          v-for="item in options"
          :key="item.value"
          :label="item.label"
          :value="item.value"
        />
      </el-select>
      <el-button text bg>
        新增key
      </el-button>
    </div>
    <div>
      <el-input placeholder="搜索" />
    </div>
  </div>
</template>

<style lang="css" scoped>
</style>
