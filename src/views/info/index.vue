<script setup lang="ts">
import { TabsProps } from '@/store/tabs'
import { useUiState } from '@/store/ui'
import { Keyspace } from '@/types/redis'
import { parseKeyspaces } from '@/util'
import { invoke } from '@tauri-apps/api'

interface InfoProps {
  tabItem: TabsProps
}

const props = defineProps<InfoProps>()
const uiState = useUiState()
const info = ref<Record<string, string>>({})
const autoRefresh = ref(false)
const keyspaces = ref<Keyspace[]>([])
const search = ref('')

const fetchInfo = async () => {
  try {
    const isConnection = await invoke<boolean>('is_connection', { id: props.tabItem.id })
    if (!isConnection) {
      return
    }
    // 获取客户端信息
    const redisInfo = await invoke<Record<string, string>>('get_info', { id: props.tabItem.id })
    info.value = redisInfo
    keyspaces.value = parseKeyspaces(redisInfo)
  } catch (error) {
    ElMessage.error(error as string)
  }
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

const keyspaceData = computed(() => keyspaces.value.filter(key => key.len !== 0))

const filterData = computed(() => Object.keys(unref(info))
  .filter(key => key.includes(unref(search)) || info.value[key].includes(unref(search)))
  .map(key => ({ key, value: info.value[key] })))
</script>

<template>
  <div flex flex-col gap-y-6 overflow-hidden>
    <!-- 自动刷新 -->
    <el-space>
      <el-tag>
        自动刷新
      </el-tag>
      <el-switch v-model="autoRefresh" />
    </el-space>
    <!-- 状态信息 -->
    <el-row :gutter="24">
      <el-col :span="24" :md="12" :lg="8">
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
              <el-tag type="success">
                {{ info.redis_version }}
              </el-tag>
            </el-space>
            <el-space overflow-hidden>
              <el-tag>OS:</el-tag>
              <el-tag type="success">
                <el-tooltip :content="info.os" :effect="uiState.theme === 'dark' ? 'light' : 'dark'">
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

      <el-col :span="24" :md="12" :lg="8">
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
              <el-tag type="success">
                {{ info.used_memory_human }}
              </el-tag>
            </el-space>
            <el-space>
              <el-tag>内存占用峰值:</el-tag>
              <el-tag type="success">
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

      <el-col :span="24" :md="12" :lg="8">
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
              <el-tag type="success">
                {{ info.connected_clients }}
              </el-tag>
            </el-space>
            <el-space>
              <el-tag>历史连接数:</el-tag>
              <el-tag type="success">
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

    <el-row>
      <el-col>
        <el-card>
          <template #header>
            <el-space>
              <i class="carbon:text-link-analysis" />
              <p>键值统计</p>
            </el-space>
          </template>
          <el-table :data="keyspaceData">
            <el-table-column label="DB" prop="db" sortable />
            <el-table-column label="Keys" prop="len" sortable />
            <el-table-column label="Expires" prop="expires" sortable />
            <el-table-column label="Avg TTL" prop="avg_ttl" sortable />
          </el-table>
        </el-card>
      </el-col>
    </el-row>

    <el-row>
      <el-col>
        <el-card>
          <template #header>
            <el-space w-full justify-between>
              <el-space>
                <i class="mdi:information" />
                <p>Redis信息集合</p>
              </el-space>

              <el-input v-model="search" placeholder="搜索">
                <template #suffix>
                  <i class="ant-design:search-outlined" />
                </template>
              </el-input>
            </el-space>
          </template>

          <el-table :data="filterData">
            <el-table-column label="Key" prop="key" sortable />
            <el-table-column label="Value" prop="value" sortable />
          </el-table>
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
