<script setup lang="ts">


</script>

<template>
  <div
    h-full
    w-full
    py4
  >
    <el-tabs
      type="border-card"
      w-full
      closable
    >
      <el-tab-pane label="Route">
        Route
      </el-tab-pane>
      <el-tab-pane label="Config">
        Config
      </el-tab-pane>
      <el-tab-pane label="Role">
        Role
      </el-tab-pane>
      <el-tab-pane label="Task">
        Task
      </el-tab-pane>
    </el-tabs>
  </div>
</template>

<style lang="css" scoped>
:deep(.el-tabs__content) {
  display: none;
}
</style>
