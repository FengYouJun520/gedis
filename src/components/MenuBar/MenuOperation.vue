<script setup lang="ts">
import { useConfig } from './useConfig'

const treeKeysOps = useConfig()
const selectDB = ref(treeKeysOps?.db || 0)

const createOptions = () => {
  const result = []
  for (let i = 0; i < 16; i++) {
    result.push({
      label: `DB${i}`,
      value: i,
    })
  }

  return result
}

const options = createOptions()

const handleChange = (val: number) => {
  treeKeysOps?.changeDb(val)
}
</script>

<template>
  <div flex flex-col gap-y2 px2 my2>
    <div flex items-center gap-x2>
      <el-select
        v-model="selectDB"
        placeholder="Select"
        size="large"
        @change="handleChange"
      >
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
