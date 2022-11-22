<script setup lang="ts">
import { useRedis } from '@/store/redis'
import { useTabs } from '@/store/tabs'
import { RedisConfig } from '@/types/redis'
import { invoke } from '@tauri-apps/api'
import { useConfig } from './useConfig'

interface MenuOperationProps {
  config: RedisConfig
}

const props = defineProps<MenuOperationProps>()
const router = useRouter()
const configState = useRedis()
const configOps = useConfig()
const tabsState = useTabs()

const handleClick = (_event: MouseEvent) => {
}

const handleHome = async () => {
  configOps?.connection(props.config)
}

const handleClose = async () => {
  const isConnection = await invoke<boolean>('is_connection', { id: props.config.id })
  if (!isConnection) {
    return
  }

  ElMessageBox.confirm('确定要关闭连接吗?', '关闭连接', {
    type: 'warning',
    confirmButtonText: '确定',
    cancelButtonText: '取消',
  }).then(async () => {
    try {
      await configOps?.disConnection(props.config.id)
      tabsState.removeTabById(props.config.id)
    } catch (error) {
      ElMessage.error(error as string)
    }
  })
    .catch(() => {})
}

const editLoading = ref(false)
const configModel = ref<RedisConfig>({ ...props.config })
const visibleEdit = ref(false)
const visibleEditDialog = async () => {
  try {
    const isConnection = await invoke('is_connection', { id: props.config.id })
    if (!isConnection) {
      visibleEdit.value = true
      configModel.value = { ...props.config }
      tabsState.removeTab(props.config.id)
      return
    }

    ElMessageBox.confirm('需要关闭当前连接才能进行编辑操作,是否继续?', {
      type: 'warning',
      confirmButtonText: '确定',
      cancelButtonText: '取消',
    }).then(async () => {
      await configOps?.disConnection(props.config.id)
      configModel.value = { ...props.config }
      tabsState.removeTabById(props.config.id)
      visibleEdit.value = true
    })
      .catch(() => {})
  } catch (error) {
    ElMessage.error(error as string)
  }
}

const handleEditConfigConfirm = () => {
  configState.editConfig(unref(configModel.value))
  visibleEdit.value = false
  editLoading.value = false
}

const handleCancel = () => {
  visibleEdit.value = false
  editLoading.value = false
  configModel.value = { ...configOps!.config }
}

const handleTestConnection = async () => {
  try {
    editLoading.value = true
    await invoke('test_connection', { config: configModel.value })
    ElMessage.success('连接成功')
  } catch (error) {
    ElMessage.error(error as string)
  } finally {
    editLoading.value = false
  }
}

const handleCommand = (command: string) => {
  switch (command) {
  case 'close':
    handleClose()
    break
  case 'edit':
    visibleEditDialog()
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

    <el-dialog
      v-model="visibleEdit"
      title="编辑连接"
      width="50%"
      append-to-body
      @close="handleCancel"
    >
      <el-form
        :model="configModel"
        label-position="top"
      >
        <el-row :gutter="24">
          <el-col :span="12">
            <el-form-item label="地址">
              <el-input
                v-model="configModel.host"
                placeholder="地址"
              />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="端口号">
              <el-input-number
                v-model="configModel.port"
                placeholder="端口号"
              />
            </el-form-item>
          </el-col>
        </el-row>

        <el-row :gutter="24">
          <el-col :span="12">
            <el-form-item label="用户名">
              <el-input
                v-model="configModel.username"
                placeholder="用户名"
              />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="密码">
              <el-input
                v-model="configModel.password"
                type="password"
                placeholder="密码"
              />
            </el-form-item>
          </el-col>
        </el-row>

        <el-row :gutter="24">
          <el-col :span="12">
            <el-form-item label="名称">
              <el-input
                v-model="configModel.name"
                placeholder="名称"
              />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="分隔符">
              <el-input
                v-model="configModel.split"
                placeholder="分隔符"
              />
            </el-form-item>
          </el-col>
        </el-row>
      </el-form>

      <template #footer>
        <div
          flex
          justify-between
          items-center
        >
          <el-button
            :loading="editLoading"
            text
            bg
            @click="handleTestConnection"
          >
            测试连接
            <template #icon>
              <i class="emojione:rocket" />
            </template>
          </el-button>
          <div
            flex
            items-center
          >
            <el-button
              text
              bg
              @click="handleCancel"
            >
              取消
            </el-button>
            <el-button
              type="primary"
              @click="handleEditConfigConfirm"
            >
              确认
            </el-button>
          </div>
        </div>
      </template>
    </el-dialog>
  </div>
</template>

<style lang="css" scoped>
</style>
