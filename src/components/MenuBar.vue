<script setup lang="ts">
import MenuHeader from '@/components/MenuHeader.vue'
import { useUiState } from '@/store/ui'
import { useResizeAsideWidth } from './useResizeAsideWidth'
import { useRedis } from '@/store/redis'
import Connection from '@/components/Connection.vue'

const uiState = useUiState()
const redisState = useRedis()
const resizeWidth = computed(() => `${uiState.asideWidth}px`)

const resizeRef = ref<HTMLDivElement|null>(null)
const handleMouse = (event: MouseEvent) => {
  uiState.setAsideWidth(event.clientX)
}

const { pressed } = useResizeAsideWidth({ target: resizeRef, touch: false }, handleMouse)
</script>

<template>
  <el-aside
    h-full
    :width="`${uiState.asideWidth}px`"
    :class="{ 'select-none': pressed }"
  >
    <div flex flex-col h-full>
      <!-- 头部 -->
      <MenuHeader />
      <!-- 内容 -->
      <el-scrollbar>
        <Connection v-for="config in redisState.configs" :key="config.id" :config="config" />
      </el-scrollbar>
    </div>

    <!-- 拖动条 -->
    <i
      ref="resizeRef"
      class="aside-resize akar-icons:two-line-vertical"
    />
  </el-aside>
</template>

<style lang="css" scoped>
.el-aside {
  border-right: solid 1px var(--el-menu-border-color)
}

.aside-resize {
  @apply fixed h-full z99;
  border: 1px solid var(--el-border-color);
  width: 16px;
  height: 16px;
  top: 50%;
  left: v-bind(resizeWidth);
}

.aside-resize:hover {
  @apply cursor-move;
}
</style>
