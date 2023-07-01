<script setup lang="ts">
import { TabsProps } from '@/store/tabs'
import { useUiState } from '@/store/ui'
import { Keyspace } from '@/types/redis'
import { parseKeyspaces } from '@/util'
import { invoke } from '@tauri-apps/api'
import { TableColumn } from 'naive-ui/es/data-table/src/interface'

interface InfoProps {
  tabItem: TabsProps
}

const props = defineProps<InfoProps>()

const keyspaceColumns: TableColumn[] = [
  {
    title: 'DB',
    key: 'db',
  },
  {
    title: 'Keys',
    key: 'len',
  },
  {
    title: 'Expires',
    key: 'expires',
  },
  {
    title: 'Avg TTL',
    key: 'avg_ttl',
  },
]

const infoListColumns: TableColumn[] = [
  {
    title: 'Key',
    key: 'key',
    ellipsis: {
      tooltip: true,
    },
  },
  {
    title: 'Value',
    key: 'value',
  },
]

const message = useMessage()
const id = ref(props.tabItem.id)
const info = ref<Record<string, string>>({})
const autoRefresh = ref(false)
const keyspaces = ref<Keyspace[]>([])
const search = ref('')

let timer:number
const fetchInfo = async () => {
  try {
    // 获取客户端信息
    const redisInfo = await invoke<Record<string, string>>('get_info', { id: unref(id) })
    info.value = redisInfo
    keyspaces.value = parseKeyspaces(redisInfo)
  } catch (error) {
    message.error(error as string)
    autoRefresh.value = false
    if (timer) {
      clearInterval(timer)
    }
  }
}


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
  if (timer) {
    clearInterval(timer)
  }
})

const keyspaceData = computed(() => unref(keyspaces).filter(key => key.len !== 0))

const infoListData = computed(() => Object.keys(unref(info))
  .filter(key => key.includes(unref(search)) || unref(info)[key].includes(unref(search)))
  .map(key => ({ key, value: unref(info)[key] })))
</script>

<template>
  <div flex flex-col gap-y-6 overflow-hidden>
    <!-- 自动刷新 -->
    <n-space ml-6>
      <n-tag type="primary">
        自动刷新
      </n-tag>
      <n-switch v-model:value="autoRefresh" />
    </n-space>
    <!-- 状态信息 -->
    <n-card embedded>
      <n-grid :cols="3" :x-gap="24" :y-gap="24" item-responsive responsive="screen">
        <n-gi span="3 m:1">
          <n-card>
            <template #header>
              <n-space>
                <i class="ant-design:cloud-server-outlined" />
                <span>服务器</span>
              </n-space>
            </template>

            <n-space vertical>
              <n-space :wrap="false">
                <n-tag type="info">
                  Redis版本:
                </n-tag>
                <n-tag type="primary">
                  {{ info.redis_version }}
                </n-tag>
              </n-space>
              <n-space overflow-hidden :wrap="false">
                <n-tag type="info">
                  OS:
                </n-tag>
                <n-tooltip :content="info.os">
                  <template #trigger>
                    <n-tag truncated type="primary">
                      {{ info.os }}
                    </n-tag>
                  </template>
                  {{ info.os }}
                </n-tooltip>
              </n-space>
              <n-space :wrap="false">
                <n-tag type="info">
                  进程ID:
                </n-tag>
                <n-tag type="success">
                  {{ info.process_id }}
                </n-tag>
              </n-space>
            </n-space>
          </n-card>
        </n-gi>

        <n-gi span="3 m:1">
          <n-card>
            <template #header>
              <n-space>
                <i class="mdi:memory" />
                <span>内存</span>
              </n-space>
            </template>

            <n-space vertical align="start">
              <n-space :wrap="false">
                <n-tag type="info">
                  已用内存:
                </n-tag>
                <n-tag type="success">
                  {{ info.used_memory_human }}
                </n-tag>
              </n-space>
              <n-space :wrap="false">
                <n-tag type="info">
                  内存占用峰值:
                </n-tag>
                <n-tag type="success">
                  {{ info.used_memory_peak_human }}
                </n-tag>
              </n-space>
              <n-space :wrap="false">
                <n-tag type="info">
                  Lua占用内存:
                </n-tag>
                <n-tag type="success">
                  {{ info.used_memory_lua_human }}
                </n-tag>
              </n-space>
            </n-space>
          </n-card>
        </n-gi>

        <n-gi span="3 m:1">
          <n-card>
            <template #header>
              <n-space>
                <i class="carbon:status-partial-fail" />
                <span>状态</span>
              </n-space>
            </template>

            <n-space vertical alignment="start">
              <n-space :wrap="false">
                <n-tag type="info">
                  客户端连接数:
                </n-tag>
                <n-tag type="success">
                  {{ info.connected_clients }}
                </n-tag>
              </n-space>
              <n-space :wrap="false">
                <n-tag type="info">
                  历史连接数:
                </n-tag>
                <n-tag type="success">
                  {{ info.total_connections_received }}
                </n-tag>
              </n-space>
              <n-space :wrap="false">
                <n-tag type="info">
                  历史命令数:
                </n-tag>
                <n-tag type="success">
                  {{ info.total_commands_processed }}
                </n-tag>
              </n-space>
            </n-space>
          </n-card>
        </n-gi>
      </n-grid>

      <n-card embedded>
        <template #header>
          <n-space>
            <i class="carbon:text-link-analysis" />
            <span>键值统计</span>
          </n-space>
        </template>
        <n-data-table :data="keyspaceData" :columns="keyspaceColumns" />
      </n-card>

      <n-card embedded>
        <template #header>
          <n-space justify="space-between">
            <n-space>
              <i class="mdi:information" />
              <span>Redis信息集合</span>
            </n-space>

            <n-input v-model:value="search" placeholder="搜索" clearable>
              <template #suffix>
                <i class="ant-design:search-outlined" />
              </template>
            </n-input>
          </n-space>
        </template>

        <n-data-table :data="infoListData" :columns="infoListColumns" />
      </n-card>
    </n-card>
  </div>
</template>

<style lang="css" scoped>
</style>
