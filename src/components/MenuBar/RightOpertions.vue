<script setup lang="ts">
import { RedisConfig } from '@/types/redis'

interface SubMenuTitleProps {
  config: RedisConfig
}

defineProps<SubMenuTitleProps>()

const handleClick = (event: MouseEvent) => {
  event.stopPropagation()
}
</script>

<template>
  <div flex items-center gap-x1>
    <i
      class="mdi:home w20px h20px hover:(text-[var(--el-menu-hover-text-color)])"
      @click="handleClick"
    />
    <i
      class="mdi:console w20px h20px hover:(text-[var(--el-menu-hover-text-color)])"
      @click="handleClick"
    />
    <i
      class="material-symbols:refresh w20px h20px hover:(text-[var(--el-menu-hover-text-color)])"
      @click="handleClick"
    />
    <el-dropdown>
      <i
        class="icon-park-twotone:more-app w16px h16px hover:(text-[var(--el-menu-hover-text-color)])"
        @click="handleClick"
      />
      <template #dropdown>
        <el-dropdown-menu>
          <el-dropdown-item>Action 1</el-dropdown-item>
          <el-dropdown-item>Action 2</el-dropdown-item>
          <el-dropdown-item>Action 3</el-dropdown-item>
          <el-dropdown-item disabled>
            Action 4
          </el-dropdown-item>
          <el-dropdown-item divided>
            Action 5
          </el-dropdown-item>
        </el-dropdown-menu>
      </template>
    </el-dropdown>
  </div>
</template>

<style lang="css" scoped>
</style>
