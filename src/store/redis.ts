import { RedisConfig } from '@/types/redis'

interface RedisState {
  configs: RedisConfig[]
}

export const useRedis = defineStore('redis', {
  state: () : RedisState => ({
    configs: [],
  }),
  actions: {
    addConfig(config: RedisConfig) {
      this.configs.push({ ...config })
    },
    removeConfig(id: string) {
      const index = this.configs.findIndex(c => c.id === id)
      if (index === -1) {
        return
      }

      this.configs.splice(index, 1)
    },
    getConfig(id: string) {
      return this.configs.find(c => c.id === id)
    },
  },
  getters: {
    getConfigs: state => state.configs,
  },
  persist: true,
})
