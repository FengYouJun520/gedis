import { RedisConfig } from '@/types/redis'
import { InjectionKey, Ref } from 'vue'

interface TreeKeystOps {
  config: RedisConfig
  treeKeys: Ref<string[]>
  db: Ref<number>
  changeDb: (db: number) => void
}


export const TreeKeysKey = Symbol() as InjectionKey<TreeKeystOps>

export const createTreeKeysContext = (ops: TreeKeystOps) => {
  provide(TreeKeysKey, ops)
}

export const useTreeKeys = () => inject(TreeKeysKey)
