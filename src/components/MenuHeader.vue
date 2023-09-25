<script setup lang="ts">
import { RedisConfig } from '@/types/redis'
import { useRedis } from '@/store/redis'
import { v4 } from 'uuid'
import conOpsApi from '@/apis/conn_ops'
import { useUiState } from '@/store/ui'
import { useMitt } from '@/useMitt'
import { useThemeVars, InputInst } from 'naive-ui'
import { UseVirtualList } from '@vueuse/components'

const initConfig: RedisConfig = {
  id: '',
  name: 'localhost@6379',
  host: '127.0.0.1',
  port: 6379,
  split: ':',
  cluster: false,
}


const message = useMessage()
const uiState = useUiState()
const configState = useRedis()
const mitt = useMitt()
const themeVars = useThemeVars()
const visibleNewConn = ref(false)
const loading = ref(false)
const configData = ref<RedisConfig>({ ...initConfig })
const logs = ref<string[]>([])
const visibleLog = ref(false)
const focusRef = ref<InputInst>()
const borderColor = computed(() => themeVars.value.borderColor)

const virtualRef = ref<typeof UseVirtualList | null>(null)
const fetchLogs = async () => {
  try {
    const res = await conOpsApi.getLogs()
    logs.value = res
  } catch (error) {
    message.error(error as string)
  }
}

const handleNewConfigBtn = async () => {
  visibleNewConn.value = true
  await nextTick()
  focusRef.value?.focus()
}

const handleNewConfigConfirm = () => {
  configData.value.id = v4()
  configState.addConfig(unref(configData))
  visibleNewConn.value = false
  loading.value = false
  configData.value = { ...initConfig }
}

const handleTestConnection = async () => {
  try {
    loading.value = true
    await conOpsApi.testConnection(configData.value)
    message.success('连接成功')
  } catch (error) {
    message.error(error as string)
  } finally {
    loading.value = false
  }
}

const handleCancel = () => {
  visibleNewConn.value = false
  loading.value = false
  configData.value = { ...initConfig }
}

const settingVisible = ref(false)
const handleSettingBtn = () => {
  settingVisible.value = true
}

const handleLogs = async () => {
  await fetchLogs()
  visibleLog.value = true
  await nextTick()
  virtualRef.value?.scrollTo(logs.value.length - 1)
}

const handleScrollLog = () => {
  virtualRef.value?.scrollTo(logs.value.length - 1)
}

mitt.on('clearLogs', async () => {
  await clearLogs()
})

// 自动滚动到底部
const autoScrollBottom = ref(false)
// 实时获取日志
const syncLog = ref(false)

watchEffect(cleanup => {
  let timer: number|undefined
  if (unref(visibleLog) && unref(syncLog)) {
    timer = setInterval(async () => {
      await fetchLogs()
      unref(autoScrollBottom) && handleScrollLog()
    }, 2000)
  }
  cleanup(() => timer && clearInterval(timer)
  )
})

onUnmounted(() => {
  mitt.off('clearLogs')
})

const clearLogs = async () => {
  try {
    await conOpsApi.clearLogs()
    logs.value = []
    visibleLog.value = false
  } catch (error) {
    message.error(error as string)
  }
}
</script>

<template>
  <div
    class="r-header"
    flex justify-center items-center
    gap-x-2 p-4
    sticky top-0 z-10
  >
    <n-button
      type="primary"
      flex-1
      @click="handleNewConfigBtn"
    >
      <template #icon>
        <i class="ic:round-add-circle" />
      </template>
      <span text-base>新建连接</span>
    </n-button>
    <n-space>
      <n-tooltip :delay="500">
        设置
        <template #trigger>
          <n-button size="small" tertiary @click="handleSettingBtn">
            <template #icon>
              <i class="material-symbols:settings" />
            </template>
          </n-button>
        </template>
      </n-tooltip>
      <n-tooltip :delay="500">
        日志
        <template #trigger>
          <n-button size="small" tertiary @click="handleLogs">
            <template #icon>
              <i class="mdi:clock-minus-outline" />
            </template>
          </n-button>
        </template>
      </n-tooltip>
    </n-space>

    <n-modal
      v-model:show="visibleNewConn"
      title="新建连接"
      :auto-focus="false"
      class="w-[60%]!"
      preset="dialog"
      :mask-closable="false"
      @after-leave="handleCancel"
    >
      <n-form
        :model="configData"
        label-placement="top"
      >
        <n-grid :cols="2" :x-gap="24" responsive="screen" item-responsive>
          <n-form-item-gi span="2 m:1" label="地址">
            <n-input ref="focusRef" v-model:value="configData.host" placeholder="localhost" />
          </n-form-item-gi>
          <n-form-item-gi span="2 m:1" label="端口号">
            <n-input-number
              v-model:value="configData.port"
              placeholder="6379"
              :min="1024"
              max="65535"
            />
          </n-form-item-gi>

          <n-form-item-gi span="2 m:1" label="用户名">
            <n-input v-model:value="configData.username" placeholder="ACL(redis >= 6.0)" />
          </n-form-item-gi>
          <n-form-item-gi span="2 m:1" label="密码">
            <n-input v-model:value="configData.password" type="password" show-password-on="click" placeholder="密码" />
          </n-form-item-gi>

          <n-form-item-gi span="2 m:1" label="名称">
            <n-input v-model:value="configData.name" placeholder="localhost@6379" />
          </n-form-item-gi>
          <n-form-item-gi span="2 m:1" label="分隔符">
            <n-input v-model:value="configData.split" placeholder=":" disabled />
          </n-form-item-gi>

          <n-form-item-gi span="2 m:1" label="集群" label-placement="left">
            <n-checkbox v-model:checked="configData.cluster" />
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
          <n-button
            :loading="loading"
            secondary
            @click="handleTestConnection"
          >
            测试连接
            <template #icon>
              <i class="emojione:rocket" />
            </template>
          </n-button>
          <n-space>
            <n-button
              secondary
              @click="handleCancel"
            >
              取消
            </n-button>
            <n-button
              type="primary"
              @click="handleNewConfigConfirm"
            >
              确认
            </n-button>
          </n-space>
        </div>
      </template>
    </n-modal>

    <n-modal
      v-model:show="settingVisible"
      title="设置"
      preset="dialog"
      :auto-focus="false"
      class="w-[60%!]"
      :style="{
        '--theme-hover-color': borderColor,
      }"
      @after-leave="handleCancel"
    >
      <div flex items-center space-x-4 justify-around>
        <div
          :style="{
            backgroundColor: uiState.theme === 'light' ? themeVars.borderColor : undefined,
          }"
          hover="bg-[var(--theme-hover-color)]"
          rounded
          cursor-pointer
          transition-background-color
          duration-200
          p-4
          @click="uiState.changeTheme('light')"
        >
          <i class="material-symbols:sunny" w80px h80px />
        </div>
        <div
          :style="{
            backgroundColor: uiState.theme === 'system' ? borderColor : undefined,
          }"
          hover="bg-[var(--theme-hover-color)]"
          rounded
          cursor-pointer
          transition-background-color
          duration-200
          p-4
          @click="uiState.changeTheme('system')"
        >
          <i class="material-symbols:brightness-auto-outline" w80px h80px />
        </div>
        <div
          class="theme-hover"
          :style="{
            backgroundColor: uiState.theme === 'dark' ? borderColor : undefined,
          }"
          hover="bg-[var(--theme-hover-color)]"
          rounded
          cursor-pointer
          transition-background-color
          duration-200
          p-4
          @click="uiState.changeTheme('dark')"
        >
          <i class="bi:moon-stars-fill" w80px h80px />
        </div>
      </div>
    </n-modal>
    <n-modal
      v-model:show="visibleLog"
      preset="card"
      title="日志"
      :auto-focus="false"
      positive-text="确认"
      class="w-[70%]!"
      :block-scroll="false"
      @after-enter="handleScrollLog"
    >
      <!-- 日志组件 -->
      <use-virtual-list
        ref="virtualRef"
        :list="logs"
        height="50vh"
        :options="{ itemHeight: 30 }"
      >
        <template #default="props">
          <n-code style="font-size: 18px;" :code="props.data" language="redis-log" />
        </template>
      </use-virtual-list>

      <div flex-1 flex items-center justify-between>
        <n-space :size="24" align="center">
          <n-tooltip>
            自动滚动都底部
            <template #trigger>
              <div>
                <span>滚动：</span>
                <n-switch v-model:value="autoScrollBottom" />
              </div>
            </template>
          </n-tooltip>
          <n-tooltip>
            实时同步日志
            <template #trigger>
              <div>
                <span>实时：</span>
                <n-switch v-model:value="syncLog" />
              </div>
            </template>
          </n-tooltip>
          <span>行数：{{ logs.length }}</span>
        </n-space>
        <n-space>
          <n-button type="error" ghost @click="clearLogs">
            清空日志
          </n-button>
        </n-space>
      </div>
    </n-modal>
  </div>
</template>

<style lang="css" scoped>
.r-header {
  background-color: v-bind("themeVars.bodyColor");
}
</style>
