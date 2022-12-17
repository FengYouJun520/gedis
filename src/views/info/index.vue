<script setup lang="ts">
import { TabsProps } from '@/store/tabs'
import { invoke } from '@tauri-apps/api'

interface InfoProps {
  tabItem: TabsProps
}

const props = defineProps<InfoProps>()
const info = ref<Record<string, string>>({})
const autoRefresh = ref(false)


const fetchInfo = async () => {
  const isConnection = await invoke<boolean>('is_connection', { id: props.tabItem.id })
  if (!isConnection) {
    return
  }
  // 连接成功
  // 获取客户端信息
  const redisInfo = await invoke<Record<string, string>>('get_info', { id: props.tabItem.id })
  info.value = redisInfo
  console.log(redisInfo)
}

let timer:number
watchEffect(async () => {
  if (unref(autoRefresh)) {
    clearInterval(timer)
    timer = setInterval(fetchInfo, 2000)
  } else {
    clearInterval(timer)
    await fetchInfo()
  }
})

onUnmounted(() => {
  clearInterval(timer)
})
</script>

<template>
  <div>
    <!-- 状态信息 -->
    <el-row :gutter="16">
      <el-col :span="8">
        <el-card>
          <template #header>
            <el-space>
              <span>
                <i class="ant-design:cloud-server-outlined" />
              </span>
              <span>服务器</span>
            </el-space>
          </template>

          <el-space direction="vertical" alignment="start">
            <el-space>
              <el-tag>Redis版本:</el-tag>
              <el-tag type="success" size="large">
                {{ info.redis_version }}
              </el-tag>
            </el-space>
            <el-space>
              <el-tag>OS:</el-tag>
              <el-tag type="success" size="large">
                <el-tooltip :content="info.os">
                  {{ info.os }}
                </el-tooltip>
              </el-tag>
            </el-space>
            <el-space>
              <el-tag>进程ID:</el-tag>
              <el-tag type="success">
                {{ info.process_id }}
              </el-tag>
            </el-space>
          </el-space>
        </el-card>
      </el-col>

      <el-col :span="8">
        <el-card>
          <template #header>
            <el-space>
              <span>
                <i class="mdi:memory" />
              </span>
              <span>内存</span>
            </el-space>
          </template>

          <el-space direction="vertical" alignment="start">
            <el-space>
              <el-tag>已用内存:</el-tag>
              <el-tag type="success" size="large">
                {{ info.used_memory_human }}
              </el-tag>
            </el-space>
            <el-space>
              <el-tag>内存占用峰值:</el-tag>
              <el-tag type="success" size="large">
                {{ info.used_memory_peak_human }}
              </el-tag>
            </el-space>
            <el-space>
              <el-tag>Lua占用内存:</el-tag>
              <el-tag type="success">
                {{ info.used_memory_lua_human }}
              </el-tag>
            </el-space>
          </el-space>
        </el-card>
      </el-col>

      <el-col :span="8">
        <el-card>
          <template #header>
            <el-space>
              <span>
                <i class="carbon:status-partial-fail" />
              </span>
              <span>服务器</span>
            </el-space>
          </template>

          <el-space direction="vertical" alignment="start">
            <el-space>
              <el-tag>客户端连接数:</el-tag>
              <el-tag type="success" size="large">
                {{ info.connected_clients }}
              </el-tag>
            </el-space>
            <el-space>
              <el-tag>历史连接数:</el-tag>
              <el-tag type="success" size="large">
                {{ info.total_connections_received }}
              </el-tag>
            </el-space>
            <el-space>
              <el-tag>历史命令数:</el-tag>
              <el-tag type="success">
                {{ info.total_commands_processed }}
              </el-tag>
            </el-space>
          </el-space>
        </el-card>
      </el-col>
    </el-row>
  </div>
</template>

<style lang="css" scoped>
.server {
  width: 100%;
  overflow-x: hidden;
  text-overflow: ellipsis;
}
</style>
