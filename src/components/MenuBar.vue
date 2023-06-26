<script setup lang="ts">
import MenuHeader from '@/components/MenuHeader.vue'
import { useUiState } from '@/store/ui'
import { useResizeAsideWidth } from './useResizeAsideWidth'
import { useRedis } from '@/store/redis'
import Connection from '@/components/Connection.vue'
import { useThemeVars } from 'naive-ui'

const themeVars = useThemeVars()
const uiState = useUiState()
const redisState = useRedis()
const resizeWidth = computed(() => `${uiState.asideWidth}px`)

const resizeRef = ref<HTMLDivElement>()
const handleMouse = (event: MouseEvent) => {
  uiState.setAsideWidth(event.clientX)
}

const { pressed } = useResizeAsideWidth(handleMouse, { target: resizeRef, touch: false })
</script>

<template>
  <n-layout-sider
    h-full
    :width="uiState.asideWidth"
    :class="{ 'select-none': pressed }"
    :native-scrollbar="false"
    class="overscroll-behavior-none"
  >
    <!-- 头部 -->
    <menu-header />
    <!-- 内容 -->
    <connection v-for="config in redisState.configs" :key="config.id" :config="config" />

    <!-- 拖动条 -->
    <i
      ref="resizeRef"
      class="aside-resize akar-icons:two-line-vertical"
    />
  </n-layout-sider>
</template>

<style lang="css" scoped>
.n-layout-sider {
  border-right: solid 1px v-bind("themeVars.borderColor");
  transition: none;
  scroll-behavior: none;
}

.aside-resize {
  @apply fixed h-full z99;
  border: 1px solid v-bind("themeVars.borderColor");
  width: 16px;
  height: 16px;
  top: 50%;
  left: v-bind(resizeWidth);
}

.aside-resize:hover {
  @apply cursor-move;
}
</style>
