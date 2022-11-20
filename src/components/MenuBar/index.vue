<script setup lang="ts">
import MenuHeader from '@/components/MenuBar/MenuHeader.vue'
import SubMenuTitle from '@/components/MenuBar/SubMenuTitle.vue'
import MenuOperation from '@/components/MenuBar/MenuOperation.vue'
import KeyList from '@/components/MenuBar/KeyList.vue'
import { useUiState } from '@/store/ui'

const uiState = useUiState()

const resizeWidth = computed(() => `${uiState.asideWidth}px`)

const handleOpen = (index: string, indexPath: string) => {
  console.log(`open index: ${index}, indexPath: ${indexPath}`)
}

const handleclose = (index: string, indexPath: string) => {
  console.log(`close index: ${index}, indexPath: ${indexPath}`)
}


const handleChange = (value: any) => {
  console.log(value)
}
const resizeRef = ref<HTMLDivElement|null>(null)
const { x } = useMouse({ touch: false })
const { pressed } = useMousePressed({ target: resizeRef })

watch(x, () => {
  if (unref(pressed)) {
    uiState.setAsideWidth(unref(x))
  }
})
</script>

<template>
  <el-aside h-full :width="`${uiState.asideWidth}px`">
    <div flex flex-col h-full>
      <!-- 头部 -->
      <MenuHeader />
      <!-- 内容 -->
      <el-scrollbar flex-1>
        <div v-for="i in 3" :key="i">
          <el-menu
            h-full
            @select="handleChange"
            @open="handleOpen"
            @close="handleclose"
          >
            <el-sub-menu index="1">
              <!-- 标题 -->
              <template #title>
                <SubMenuTitle />
              </template>
              <!-- 操作 -->
              <MenuOperation />
              <!-- key列表 -->
              <KeyList />
            </el-sub-menu>
          </el-menu>
        </div>
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
  position: relative;
  border-right: solid 1px var(--el-menu-border-color)
}

.el-menu {
  border-right: none;
}

.aside-resize {
  @apply fixed h-full z999;
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
