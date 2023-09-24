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
    <transition-group name="list">
      <connection v-for="config in redisState.configs" :key="config.id" :config="config" />
    </transition-group>

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
  position: fixed;
  height: 100%;
  z-index: 99;
  border: 1px solid v-bind("themeVars.borderColor");
  width: 16px;
  height: 16px;
  top: 50%;
  left: v-bind(resizeWidth);
}

.aside-resize:hover {
  @apply cursor-move;
}

.list-move, /* 对移动中的元素应用的过渡 */
.list-enter-active,
.list-leave-active {
  transition: all 0.2s ease;
}

.list-enter-from,
.list-leave-to {
  opacity: 0;
  transform: translateX(30px);
}

/* 确保将离开的元素从布局流中删除
  以便能够正确地计算移动的动画。 */
.list-leave-active {
  position: absolute;
}
</style>
