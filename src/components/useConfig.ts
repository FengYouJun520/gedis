import { TabsProps } from '@/store/tabs'
import { Keyspace, RedisConfig } from '@/types/redis'
import { InjectionKey, Ref } from 'vue'

interface ConfigOps {
  config: RedisConfig
  treeKeys: Ref<any[]>
  keyspaces: Ref<Keyspace[]|Record<string, Keyspace[]>>,
  db: Ref<number>,
  connection: (config: RedisConfig, tabs?: TabsProps) => Promise<void>,
  disConnection: (id: string) => Promise<void>,
}


export const ConfigKey = Symbol() as InjectionKey<ConfigOps>

export const createConfigContext = (ops: ConfigOps) => {
  provide(ConfigKey, ops)
}

export const useConfig = () => inject(ConfigKey)
