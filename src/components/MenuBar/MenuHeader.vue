<script setup lang="ts">


</script>

<template>
  <div flex justify-center items-center gap-x4 p4>
    <el-button type="primary" text bg class="flex-1">
      <template #icon>
        <i class="ic:round-add-circle" />
      </template>
      <span class="text-base">新建连接</span>
    </el-button>
    <el-space>
      <el-button size="small" text bg>
        <template #icon>
          <i class="material-symbols:settings" />
        </template>
      </el-button>
      <el-button size="small" text bg>
        <template #icon>
          <i class="mdi:clock-minus-outline" />
        </template>
      </el-button>
    </el-space>
  </div>
</template>

<style lang="css" scoped>
</style>
