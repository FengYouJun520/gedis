import { RedisConfig } from '@/types/redis'
import { invoke } from '@tauri-apps/api'

export function testConnection(config: RedisConfig) {
  return invoke('test_connection', { config })
}

export function connection(config: RedisConfig) {
  return invoke('connection', { config })
}

export function isConnection(id: string) {
  return invoke<boolean>('is_connection', { id })
}

export function ping(id: string) {
  return invoke('ping', { id })
}

export function changeDb(id: string, db: number) {
  return invoke('change_db', { id, db })
}

export function disConnection(id: string) {
  return invoke('dis_connection', { id })
}

export function disConnectionAll() {
  return invoke('dis_connection_all')
}

export function getInfo(id: string) {
  return invoke<Record<string, any>>('get_info', { id })
}

export function getLogs() {
  return invoke<string[]>('get_logs')
}

export function clearLogs() {
  return invoke('clear_logs')
}

export default {
  testConnection,
  connection,
  ping,
  changeDb,
  clearLogs,
  disConnection,
  disConnectionAll,
  getInfo,
  getLogs,
  isConnection,
}
