<script setup lang="ts">
import { RedisConfig } from '@/types/redis'
import { useRedis } from '@/store/redis'
import { v4 } from 'uuid'
import { invoke } from '@tauri-apps/api'
import { useUiState } from '@/store/ui'

const initConfig: RedisConfig = {
  id: '',
  name: 'localhost',
  host: '127.0.0.1',
  port: 6379,
  split: ':',
}

const uiState = useUiState()
const configState = useRedis()

const visible = ref(false)
const loading = ref(false)
const configData = ref<RedisConfig>({ ...initConfig })

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
    ElMessage.success('连接成功')
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
      <span class="text-base">新建连接</span>
    </el-button>
    <el-space>
      <el-button size="small" text bg @click="handleSettingBtn">
        <template #icon>
          <i class="material-symbols:settings" />
        </template>
      </el-button>
      <el-button size="small" text bg>
        <template #icon>
          <i class="mdi:clock-minus-outline" />
        </template>
      </el-button>
    </el-space>

    <el-dialog
      v-model="visible"
      title="新建连接"
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
            <el-form-item label="地址">
              <el-input
                v-model="configData.host"
                placeholder="127.0.0.1"
              />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="端口号" w-full>
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
            <el-form-item label="用户名">
              <el-input
                v-model="configData.username"
                placeholder="ACL(redis >= 6.0)"
              />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="密码">
              <el-input
                v-model="configData.password"
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
                v-model="configData.name"
                placeholder="localhost"
              />
            </el-form-item>
          </el-col>
          <el-col :span="12">
            <el-form-item label="分隔符">
              <el-input
                v-model="configData.split"
                placeholder=":"
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
            :loading="loading"
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
              @click="handleNewConfigConfirm"
            >
              确认
            </el-button>
          </div>
        </div>
      </template>
    </el-dialog>

    <el-dialog
      v-model="settingVisible"
      title="设置"
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
        <el-button type="primary" @click="settingVisible = false">
          确定
        </el-button>
      </template>
    </el-dialog>
  </div>
</template>

<style lang="css" scoped>
</style>
