import { invoke } from '@tauri-apps/api'

export function sendTerminalCli(id:string, db: number, args?: string[]) {
  return invoke<any>('terminal', { id, db, args })
}
