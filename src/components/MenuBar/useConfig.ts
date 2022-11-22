import { RedisConfig } from '@/types/redis'
import { InjectionKey, Ref } from 'vue'

interface ConfigOps {
  config: RedisConfig
  treeKeys: Ref<string[]>
  db: Ref<number>,
  fetchTreeKeys: (id: string, db: number) => Promise<void>,
  connection: (config: RedisConfig) => Promise<void>,
  disConnection: (id: string) => Promise<void>,
  changeDb: (db: number) => void
}


export const ConfigKey = Symbol() as InjectionKey<ConfigOps>

export const createConfigContext = (ops: ConfigOps) => {
  provide(ConfigKey, ops)
}

export const useConfig = () => inject(ConfigKey)
