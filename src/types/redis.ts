export interface RedisConfig {
  id: string
  name: string
  host: string
  port: number
  username?: string
  password?: string
  split: string
}

// redis 信息对象封装
export interface Keyspace {
  db: number
  len: number
  expires: number
  avg_ttl: number
}

export interface TreeNode {
  key:string
  label: string
  value: string
  children?: TreeNode[]
}
