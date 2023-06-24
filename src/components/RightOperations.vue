<script setup lang="tsx">
import { useRedis } from '@/store/redis'
import { useTabs } from '@/store/tabs'
import { RedisConfig } from '@/types/redis'
import { useMitt } from '@/useMitt'
import { invoke } from '@tauri-apps/api'
import { v4 } from 'uuid'
import { useConfig } from './useConfig'
import { DropdownOption, NIcon, useThemeVars } from 'naive-ui'

interface MenuOperationProps {
  config: RedisConfig
}

const props = defineProps<MenuOperationProps>()
const dialog = useDialog()
const themeVars = useThemeVars()
const message = useMessage()
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

const handleTerminal = async () => {
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

  dialog.success({
    title: '关闭连接',
    content: '确定要关闭连接吗？',
    positiveText: '确定',
    negativeText: '取消',
    onPositiveClick: async () => {
      try {
        await configOps?.disConnection(unref(id))
        tabsState.removeTabById(unref(id))
      } catch (error) {
        message.error(error as string)
      }
    },
  })
}

const handleDelete = async () => {
  dialog.error({
    title: '删除连接',
    content: '确定要删除连接吗？',
    positiveText: '确定',
    negativeText: '取消',
    onPositiveClick: async () => {
      try {
        const isConnection = await invoke<boolean>('is_connection', { id: unref(id) })
        if (isConnection) {
          await configOps?.disConnection(unref(id))
        }
        tabsState.removeTabById(unref(id))
        configState.removeConfig(unref(id))
      } catch (error) {
        message.error(error as string)
      }
    },
  })
}

const editLoading = ref(false)
const configModel = ref<RedisConfig>({ ...props.config })
const visibleEdit = ref(false)
const isEdit = ref(false)

const dropdownOptions: DropdownOption[] = [
  {
    label: '关闭连接',
    key: 'close',
    icon: () => <NIcon><i class="grommet-icons:power-shutdown" /></NIcon>,
  },
  {
    label: '编辑连接',
    key: 'edit',
    icon: () => <NIcon><i class="mdi:square-edit-outline" /></NIcon>,
  },
  {
    label: '删除连接',
    key: 'delete',
    icon: () => <NIcon><i class="material-symbols:delete-outline" /></NIcon>,
  },
  {
    label: '复制连接',
    key: 'copy',
    icon: () => <NIcon><i class="material-symbols:content-copy-outline" /></NIcon>,
  },
  {
    label: '删除所有键',
    key: 'deleteKeyAll',
    icon: () => <NIcon><i class="material-symbols:warning" /></NIcon>,
  },
]

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

const visibleDialog = async (edit: boolean) => {
  try {
    isEdit.value = edit
    const isConnection = await invoke('is_connection', { id: unref(id) })
    // 如果是编辑并且没有连接
    if (!isConnection && unref(isEdit)) {
      configModel.value = { ...props.config }
      tabsState.removeTab(unref(id))
      visibleEdit.value = true
      return
    }

    if (unref(isEdit)) {
      // 编辑连接
      dialog.info({
        title: unref(isEdit) ? '编辑连接' : '复制连接',
        content: `需要关闭当前连接才能进行${unref(isEdit) ? '编辑' : '复制'}操作,是否继续?`,
        positiveText: '确定',
        negativeText: '取消',
        onPositiveClick: async () => {
          await configOps?.disConnection(unref(id))
          configModel.value = { ...props.config }
          tabsState.removeTabById(unref(id))
          visibleEdit.value = true
        },
      })
    } else {
      // 复制连接
      configModel.value = { ...props.config }
      visibleEdit.value = true
    }
  } catch (error) {
    message.error(error as string)
  }
}


const handleTestConnection = async () => {
  try {
    editLoading.value = true
    await invoke('test_connection', { config: configModel.value })
    message.success('连接成功')
  } catch (error) {
    message.error(error as string)
  } finally {
    editLoading.value = false
  }
}

const handleClearKeys = async () => {
  dialog.warning({
    title: '清空键',
    content: '你确定要清空键吗?(该操作不可逆)',
    positiveText: '确定',
    negativeText: '取消',
    onPositiveClick: async () => {
      try {
        await invoke('clear_keys', { id: unref(id), db: unref(selectDb) })

        // 删除所有相关的选项卡(DB)
        tabsState.removeTabByDb(unref(selectDb))

        mitt.emit('refresh', { id: unref(id), db: unref(selectDb) })
      } catch (error) {
        message.error(error as string)
      }
    },
  })
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
  visibleEdit.value = false
  editLoading.value = false
  isEdit.value = false
}

const handleCancel = () => {
  visibleEdit.value = false
  editLoading.value = false
  isEdit.value = false
  configModel.value = { ...configOps!.config }
}
</script>

<template>
  <div flex items-center gap-x-1 :style="{
    '--icon-hover-color': themeVars.iconColorHover,
  }">
    <i
      class="mdi:home"
      w-20px
      h-20px
      hover="text-[var(--icon-hover-color)]" @click.stop="handleHome"
    />
    <i
      class="mdi:console"
      w-20px h-20px
      hover="text-[var(--icon-hover-color)]" @click.stop="handleTerminal"
    />
    <i
      class="material-symbols:refresh"
      w-20px
      h-20px
      hover="text-[var(--icon-hover-color)]" @click.stop="handleRefresh"
    />
    <n-dropdown :options="dropdownOptions" @select="handleCommand">
      <i
        class="icon-park-twotone:more-app"
        w-16px
        h-16px
        hover="text-[var(--icon-hover-color)]"
        @click.stop="handleClick"
      />
    </n-dropdown>

    <n-modal
      v-model:show="visibleEdit"
      :auto-focus="false"
      title="编辑连接"
      preset="dialog"
      class="w-[60%]!"
      @after-leave="handleCancel"
    >
      <n-form :model="configModel">
        <n-grid :cols="2" :x-gap="24" responsive="screen" item-responsive>
          <n-form-item-gi span="2 m:1" label="地址">
            <n-input v-model:value="configModel.host" placeholder="地址" />
          </n-form-item-gi>
          <n-form-item-gi span="2 m:1" label="端口号">
            <n-input-number
              v-model:value="configModel.port"
              placeholder="端口号"
              :min="1024"
              :max="65535"
            />
          </n-form-item-gi>

          <n-form-item-gi span="2 m:1" label="用户名">
            <n-input v-model:value="configModel.username" placeholder="用户名" />
          </n-form-item-gi>
          <n-form-item-gi span="2 m:1" label="密码">
            <n-input v-model:value="configModel.password" type="password" placeholder="密码" />
          </n-form-item-gi>

          <n-form-item-gi span="2 m:1" label="名称">
            <n-input v-model:value="configModel.name" placeholder="名称" />
          </n-form-item-gi>
          <n-form-item-gi span="2 m:1" label="分隔符">
            <n-input v-model:value="configModel.split" disabled placeholder="分隔符" />
          </n-form-item-gi>
        </n-grid>
      </n-form>

      <template #action>
        <div
          flex-1
          flex
          justify-between
          items-center
        >
          <n-button :loading="editLoading" secondary @click="handleTestConnection">
            测试连接
            <template #icon>
              <i class="emojione:rocket" />
            </template>
          </n-button>
          <n-space>
            <n-button secondary @click="handleCancel">
              取消
            </n-button>
            <n-button type="primary" @click="handleConfigConfirm">
              确认
            </n-button>
          </n-space>
        </div>
      </template>
    </n-modal>
  </div>
</template>

<style lang="css" scoped>
</style>
