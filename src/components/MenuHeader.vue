<script setup lang="ts">
import { RedisConfig } from '@/types/redis'
import { useRedis } from '@/store/redis'
import { v4 } from 'uuid'
import { clipboard, invoke } from '@tauri-apps/api'
import { useUiState } from '@/store/ui'
import { useMitt } from '@/useMitt'
import type { ElScrollbar } from 'element-plus'

const initConfig: RedisConfig = {
  id: '',
  name: 'localhost',
  host: '127.0.0.1',
  port: 6379,
  split: ':',
  cluster: false,
}

const uiState = useUiState()
const configState = useRedis()
const mitt = useMitt()

const visible = ref(false)
const loading = ref(false)
const configData = ref<RedisConfig>({ ...initConfig })
const logs = ref<string[]>([])
const visibleLog = ref(false)
const scrollbarRef = ref<InstanceType<typeof ElScrollbar> | null>(null)
const ulRef = ref<HTMLUListElement | null>(null)
const scrollHeight = 300

const fetchlogs = async () => {
  try {
    const res = await invoke<string[]>('get_logs')
    logs.value = res
  } catch (error) {
    ElMessage.error(error as string)
  }
}

const handleNewConfigBtn = () => {
  visible.value = true
}

const handleNewConfigConfirm = () => {
  configData.value.id = v4()
  configState.addConfig(unref(configData))
  visible.value = false
  loading.value = false
  configData.value = { ...initConfig }
}

const handleTestConnection = async () => {
  try {
    loading.value = true
    await invoke('test_connection', { config: configData.value })
    ElMessage.success('θΏζ₯ζε')
  } catch (error) {
    ElMessage.error(error as string)
  } finally {
    loading.value = false
  }
}

const handleCancel = () => {
  visible.value = false
  loading.value = false
  configData.value = { ...initConfig }
}

const settingVisible = ref(false)
const handleSettingBtn = () => {
  settingVisible.value = true
}
const handleSettingCancel = () => {
  settingVisible.value = false
}

const handleLogs = async () => {
  await fetchlogs()
  visibleLog.value = true
}

const handleOpenLog = () => {
  nextTick(() => {
    scrollbarRef.value?.scrollTo({
      top: ulRef.value?.scrollHeight,
    })
  })
}

mitt.on('clearLogs', async () => {
  await clearLogs()
})

const autoScrollBottom = ref(false)
// ε?ζΆθ·εζ₯εΏ
const syncLog = ref(false)

watchEffect(cleanup => {
  let timer: number|undefined
  if (unref(visibleLog) && unref(syncLog)) {
    timer && clearInterval(timer)
    timer = setInterval(async () => {
      await fetchlogs()
      unref(autoScrollBottom) && handleOpenLog()
    }, 2000)
  }
  cleanup(() => timer && clearInterval(timer))
})

onUnmounted(() => {
  mitt.off('clearLogs')
})

const clearLogs = async () => {
  try {
    await invoke('clear_logs')
    logs.value = []
    visibleLog.value = false
  } catch (error) {
    ElMessage.error(error as string)
  }
}

const alertType = (log: string) => {
  const arg = log.split(/\s+/)[1]
  if (arg.match(/(.*add.*)|(.*set.*)|(.*push*)/)) {
    return 'success'
  } else if (arg.match(/(.*del.*)|(.*pop.*)/)) {
    return 'error'
  } else {
    return 'info'
  }
}

const copyCommand = (log: string) => {
  clipboard.writeText(log)
}
</script>

<template>
  <div flex justify-center items-center gap-x4 p4>
    <el-button
      type="primary"
      text bg
      class="flex-1"
      @click="handleNewConfigBtn"
    >
      <template #icon>
        <i class="ic:round-add-circle" />
      </template>
      <span class="text-base">ζ°ε»ΊθΏζ₯</span>
    </el-button>
    <el-space>
      <el-tooltip content="θ?Ύη½?" :show-after="500">
        <el-button size="small" text bg @click="handleSettingBtn">
          <template #icon>
            <i class="material-symbols:settings" />
          </template>
        </el-button>
      </el-tooltip>
      <el-tooltip content="ζ₯εΏ" :show-after="500">
        <el-button size="small" text bg @click="handleLogs">
          <template #icon>
            <i class="mdi:clock-minus-outline" />
          </template>
        </el-button>
      </el-tooltip>
    </el-space>

    <el-dialog
      v-model="visible"
      title="ζ°ε»ΊθΏζ₯"
      width="50%"
      append-to-body
      :close-on-click-modal="false"
      @close="handleCancel"
    >
      <el-form
        :model="configData"
        label-position="top"
      >
        <el-row :gutter="24">
          <el-col :span="12">
            <el-form-item label="ε°ε">
              <el-input
                v-model="configData.host"
                placeholder="127.0.0.1"
              />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="η«―ε£ε·" w-full>
              <el-input-number
                v-model="configData.port"
                placeholder="6379"
                flex-1
              />
            </el-form-item>
          </el-col>
        </el-row>

        <el-row :gutter="24">
          <el-col :span="12">
            <el-form-item label="η¨ζ·ε">
              <el-input
                v-model="configData.username"
                placeholder="ACL(redis >= 6.0)"
              />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="ε―η ">
              <el-input
                v-model="configData.password"
                type="password"
                placeholder="ε―η "
                show-password
              />
            </el-form-item>
          </el-col>
        </el-row>

        <el-row :gutter="24">
          <el-col :span="12">
            <el-form-item label="εη§°">
              <el-input
                v-model="configData.name"
                placeholder="localhost"
              />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="ειη¬¦">
              <el-input
                v-model="configData.split"
                placeholder=":"
              />
            </el-form-item>
          </el-col>
        </el-row>

        <el-row :gutter="24">
          <el-col :span="8">
            <el-form-item>
              <el-checkbox v-model="configData.cluster" label="ιηΎ€" />
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
            :loading="loading"
            text
            bg
            @click="handleTestConnection"
          >
            ζ΅θ―θΏζ₯
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
              εζΆ
            </el-button>
            <el-button
              type="primary"
              @click="handleNewConfigConfirm"
            >
              η‘?θ?€
            </el-button>
          </div>
        </div>
      </template>
    </el-dialog>

    <el-dialog
      v-model="settingVisible"
      title="θ?Ύη½?"
      width="50%"
      append-to-body
      @close="handleCancel"
    >
      <div flex items-center space-x-4 justify-around>
        <div
          :class="{ 'bg-[var(--el-border-color)]': uiState.theme === 'system' }"
          p4 hover="bg-[var(--el-border-color)] cursor-pointer"
          rounded
          transition-all
          duration-200
          @click="uiState.changeTheme('system')"
        >
          <i class="material-symbols:brightness-auto-outline" w80px h80px />
        </div>
        <div
          :class="{ 'bg-[var(--el-border-color)]': uiState.theme === 'dark' }"
          p4 hover="bg-[var(--el-border-color)] cursor-pointer"
          rounded
          transition-all
          duration-200
          @click="uiState.changeTheme('dark')"
        >
          <i class="bi:moon-stars-fill" w80px h80px />
        </div>
        <div
          :class="{ 'bg-[var(--el-border-color)]': uiState.theme === 'light' }"
          p4 hover="bg-[var(--el-border-color)] cursor-pointer"
          rounded
          transition-all
          duration-200
          @click="uiState.changeTheme('light')"
        >
          <i class="material-symbols:sunny" w80px h80px />
        </div>
      </div>

      <template #footer>
        <el-button type="primary" @click="handleSettingCancel">
          η‘?ε?
        </el-button>
      </template>
    </el-dialog>

    <el-dialog
      v-model="visibleLog"
      title="ζ₯εΏ"
      width="70%"
      append-to-body
      destroy-on-close
      @open="handleOpenLog"
      @close="visibleLog = false"
    >
      <ElScrollbar ref="scrollbarRef" :min-size="150" :height="scrollHeight">
        <ul
          ref="ulRef"
          py-0
          px-4
        >
          <li
            v-for="(log, index) in logs"
            :key="index"
            flex
            flex-col gap-y-3
          >
            <el-alert
              :closable="false"
              :type="alertType(log)"
              relative
            >
              <span text-1rem>
                {{ log.length >= 100 ? `${log.substring(0, 100)}...` : log }}
              </span>

              <el-tooltip content="ε€εΆε½δ»€" :show-after="1000">
                <el-button
                  size="small"
                  text
                  absolute
                  right-20px
                  class="top-50% -translate-y-50% cursor-pointer"
                  @click="copyCommand(log)"
                >
                  <template #icon>
                    <i class="ant-design:copy-outlined" />
                  </template>
                </el-button>
              </el-tooltip>
            </el-alert>
          </li>
        </ul>
      </ElScrollbar>

      <template #footer>
        <div flex items-center justify-between>
          <el-space :size="24">
            <el-tooltip content="θͺε¨ζ»ε¨ι½εΊι¨">
              <div>
                <span>ζ»ε¨οΌ</span>
                <el-switch v-model="autoScrollBottom" />
              </div>
            </el-tooltip>
            <el-tooltip content="ε?ζΆεζ­₯ζ₯εΏ">
              <div>
                <span>ε?ζΆοΌ</span>
                <el-switch v-model="syncLog" />
              </div>
            </el-tooltip>
          </el-space>
          <el-space>
            <el-button type="danger" text bg @click="clearLogs">
              ζΈη©Ίζ₯εΏ
            </el-button>
            <el-button type="primary" @click="visibleLog = false">
              εζΆ
            </el-button>
          </el-space>
        </div>
      </template>
    </el-dialog>
  </div>
</template>

<style lang="css" scoped>
</style>
