import { KeyInfo, KeyContentDetail, AddKeyInfo } from '@/types/redis'
import { invoke } from '@tauri-apps/api'

export function getKeyType(id:string, db: number, key: string) {
  return invoke<string>('get_key_type', { id, db, key })
}

export function delKey(id:string, db: number, key: string) {
  return invoke('del_key', { id, db, key })
}

export function delMatchKeys(id:string, db: number, matchKey: string) {
  return invoke('del_match_key', { id, db, matchKey })
}

export function delKeyByValue(id:string, db: number, key: string, value?: string) {
  return invoke('del_key_by_value', { id, db, key, value })
}

export function clearKeys(id:string, db: number) {
  return invoke('clear_keys', { id, db })
}

export function getKeysByDb(id:string, db: number) {
  return invoke<string[]>('get_keys_by_db', { id, db })
}

export function getKeyInfo(id:string, db: number, key: string) {
  return invoke<KeyInfo>('get_key_info', { id, db, key })
}

export function getKeyDetail<T = any>(id:string, db: number, key: string) {
  return invoke<KeyContentDetail<T>>('get_key_detail', { id, db, key })
}

export function renameKey(id:string, db: number, key: string, newKey: string) {
  return invoke('rename_key', { id, db, key, newKey })
}

export function setKey(id:string, db: number, keyinfo: AddKeyInfo) {
  return invoke('set_key', { id, db, keyinfo })
}

export function setKeyTTL(id:string, db: number, key: string, ttl: number) {
  return invoke('set_key_ttl', { id, db, key, ttl })
}

export default {
  getKeyType,
  delKey,
  delMatchKeys,
  delKeyByValue,
  clearKeys,
  getKeysByDb,
  getKeyInfo,
  getKeyDetail,
  renameKey,
  setKey,
  setKeyTTL,
}
