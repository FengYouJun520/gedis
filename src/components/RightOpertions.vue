<script setup lang="ts">
import { useRedis } from '@/store/redis'
import { useTabs } from '@/store/tabs'
import { RedisConfig } from '@/types/redis'
import { useMitt } from '@/useMitt'
import { invoke } from '@tauri-apps/api'
import { v4 } from 'uuid'
import { useConfig } from './useConfig'

interface MenuOperationProps {
  config: RedisConfig
}

const props = defineProps<MenuOperationProps>()
const mitt = useMitt()
const configState = useRedis()
const configOps = useConfig()
const tabsState = useTabs()
const id = computed(() => props.config.id)
const selectDb = defineModel('db', { default: 0, required: true })

const handleClick = (_event: MouseEvent) => {
}

const handleRefresh = () => {
  mitt.emit('refresh', { id: unref(id), db: unref(selectDb) })
}

const handleHome = async () => {
  await configOps?.connection(props.config)
}

const handleConsole = async () => {
  await configOps?.connection(
    props.config,
    {
      id: unref(id),
      key: `${unref(id)}-${unref(selectDb)}`,
      value: unref(id),
      name: props.config.name,
      label: `${props.config.name} | redis-cli: ${props.config.port}`,
      db: unref(selectDb),
      type: 'terminal',
      icon: 'mdi:console',
    }
  )
}

const handleClose = async () => {
  const isConnection = await invoke<boolean>('is_connection', { id: unref(id) })
  if (!isConnection) {
    return
  }

  ElMessageBox.confirm('确定要关闭连接吗?', '关闭连接', {
    type: 'warning',
    confirmButtonText: '确定',
    cancelButtonText: '取消',
  }).then(async () => {
    try {
      await configOps?.disConnection(unref(id))
      tabsState.removeTabById(unref(id))
    } catch (error) {
      ElMessage.error(error as string)
    }
  })
    .catch(() => {})
}

const handleDelete = async () => {
  ElMessageBox.confirm('确定要删除连接吗?', '删除连接', {
    type: 'warning',
    confirmButtonText: '确定',
    cancelButtonText: '取消',
  }).then(async () => {
    try {
      const isConnection = await invoke<boolean>('is_connection', { id: unref(id) })
      if (isConnection) {
        await configOps?.disConnection(unref(id))
      }
      tabsState.removeTabById(unref(id))
      configState.removeConfig(unref(id))
    } catch (error) {
      ElMessage.error(error as string)
    }
  })
    .catch(() => {})
}

const editLoading = ref(false)
const configModel = ref<RedisConfig>({ ...props.config })
const visible = ref(false)
const isEdit = ref(false)
const visibleDialog = async (edit: boolean) => {
  try {
    isEdit.value = edit
    const isConnection = await invoke('is_connection', { id: unref(id) })
    // 如果是编辑并且没有连接
    if (!isConnection && unref(isEdit)) {
      configModel.value = { ...props.config }
      tabsState.removeTab(unref(id))
      visible.value = true
      return
    }

    if (unref(isEdit)) {
      // 编辑连接
      ElMessageBox.confirm('需要关闭当前连接才能进行编辑操作,是否继续?', {
        type: 'warning',
        confirmButtonText: '确定',
        cancelButtonText: '取消',
      }).then(async () => {
        await configOps?.disConnection(unref(id))
        configModel.value = { ...props.config }
        tabsState.removeTabById(unref(id))
        visible.value = true
      })
        .catch(() => {})
    } else {
      // 复制连接
      configModel.value = { ...props.config }
      visible.value = true
    }
  } catch (error) {
    ElMessage.error(error as string)
  }
}

const handleConfigConfirm = () => {
  if (unref(isEdit)) {
    // 编辑
    configState.editConfig(unref(configModel))
  } else {
    // 复制
    configModel.value.id = v4()
    configState.addConfig(unref(configModel))
  }
  visible.value = false
  editLoading.value = false
  isEdit.value = false
}

const handleCancel = () => {
  visible.value = false
  editLoading.value = false
  isEdit.value = false
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

const handleClearKeys = async () => {
  ElMessageBox.confirm('你确定要清空键吗?(该操作不可逆)', {
    type: 'error',
    confirmButtonText: '确定',
    cancelButtonText: '取消',
  }).then(async () => {
    try {
      await invoke('clear_keys', { id: unref(id), db: unref(selectDb) })

      // 删除所有相关的选项卡(DB)
      tabsState.removeTabByDb(unref(selectDb))

      mitt.emit('refresh', { id: unref(id), db: unref(selectDb) })
    } catch (error) {
      ElMessage.error(error as string)
    }
  })
    .catch(() => {})
}

const handleCommand = async (command: string) => {
  switch (command) {
  case 'close':
    await handleClose()
    break
  case 'edit':
    await visibleDialog(true)
    break
  case 'delete':
    await handleDelete()
    break
  case 'copy':
    await visibleDialog(false)
    break
  case 'deleteKeyAll':
    await handleClearKeys()
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
      @click.stop="handleConsole"
    />
    <i
      class="material-symbols:refresh w20px h20px hover:(text-[var(--el-menu-hover-text-color)])"
      @click.stop="handleRefresh"
    />
    <el-dropdown @command="handleCommand">
      <i
        class="icon-park-twotone:more-app w16px h16px hover:(text-[var(--el-menu-hover-text-color)])"
        @click.stop="handleClick"
      />
      <template #dropdown>
        <el-dropdown-menu>
          <el-dropdown-item command="close">
            <div flex items-center>
              <i class="grommet-icons:power-shutdown" />
              <span>关闭连接</span>
            </div>
          </el-dropdown-item>
          <el-dropdown-item command="edit">
            <div flex items-center>
              <i class="mdi:square-edit-outline" />
              <span>编辑连接</span>
            </div>
          </el-dropdown-item>
          <el-dropdown-item command="delete">
            <div flex items-center>
              <i class="material-symbols:delete-outline" />
              <span>删除连接</span>
            </div>
          </el-dropdown-item>
          <el-dropdown-item command="copy">
            <div flex items-center>
              <i class="material-symbols:content-copy-outline" />
              <span>复制连接</span>
            </div>
          </el-dropdown-item>
          <el-dropdown-item command="deleteKeyAll" divided>
            <div flex items-center>
              <i class="material-symbols:warning" />
              <span>删除所有键</span>
            </div>
          </el-dropdown-item>
        </el-dropdown-menu>
      </template>
    </el-dropdown>

    <el-dialog
      v-model="visible"
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
                show-password
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

        <el-row :gutter="24">
          <el-col :span="8">
            <el-form-item>
              <el-checkbox v-model="configModel.cluster" label="集群" />
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
              @click="handleConfigConfirm"
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
