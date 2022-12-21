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

export interface AddKeyInfo {
  type: string
  key: string
  value: string
  score?: number
  field?: string
  id?: string
}

export interface KeyInfo {
  key: string
  label: string
  type: string
  ttl: number
}

export interface KeyContentDetail {
  key: string
  label: string
  type: string
  ttl: number
  size: number
  value: any
}
