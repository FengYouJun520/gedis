<script setup lang="ts">
import { invoke } from '@tauri-apps/api'
import { useConfig } from './useConfig'


const router = useRouter()
const configOps = useConfig()

const handleClick = (_event: MouseEvent) => {
}

const handleHome = async () => {
  configOps?.connection(configOps.config)
}

const handleClose = () => {
  ElMessageBox.confirm('确定要关闭连接吗?', '关闭连接', {
    type: 'warning',
    confirmButtonText: '确定',
    cancelButtonText: '取消',
  }).then(async () => {
    try {
      await configOps?.disConnection(configOps.config.id)
    } catch (error) {
      ElMessage.error(error as string)
    }
  })
    .catch(() => {})
}

const handleCommand = (command: string) => {
  switch (command) {
  case 'close':
    handleClose()
    break
  case 'edit':
    break
  case 'delete':
    break
  case 'copy':
    break
  case 'deleteKeyAll':
    break
  default:
    break
  }
}
</script>

<template>
  <div flex items-center gap-x1>
    <i
      class="mdi:home w20px h20px hover:(text-[var(--el-menu-hover-text-color)])"
      @click.stop="handleHome"
    />
    <i
      class="mdi:console w20px h20px hover:(text-[var(--el-menu-hover-text-color)])"
      @click.stop="handleClick"
    />
    <i
      class="material-symbols:refresh w20px h20px hover:(text-[var(--el-menu-hover-text-color)])"
      @click.stop="handleClick"
    />
    <el-dropdown @command="handleCommand">
      <i
        class="icon-park-twotone:more-app w16px h16px hover:(text-[var(--el-menu-hover-text-color)])"
        @click.stop="handleClick"
      />
      <template #dropdown>
        <el-dropdown-menu>
          <el-dropdown-item command="close">
            关闭连接
          </el-dropdown-item>
          <el-dropdown-item command="edit">
            编辑连接
          </el-dropdown-item>
          <el-dropdown-item command="delete">
            删除连接
          </el-dropdown-item>
          <el-dropdown-item command="copy">
            复制连接
          </el-dropdown-item>
          <el-dropdown-item command="" divided />
          <el-dropdown-item command="deleteKeyAll">
            删除所有键
          </el-dropdown-item>
        </el-dropdown-menu>
      </template>
    </el-dropdown>
  </div>
</template>

<style lang="css" scoped>
</style>
