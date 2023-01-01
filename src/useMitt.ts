import mitt, { Emitter } from 'mitt'

type Events = {
  changeDb: number
  disConnection: string
  fetchInfo: string
  searchKeyTree: {
    id: string,
    query: string
  }
  refresh: {
    id: string,
    db: number
  }
  fetchTreeKeys: {
    id: string,
    db: number
  }
  fetchKeyInfo: string
  clearLogs?: void
}

const emitter: Emitter<Events> = mitt<Events>()

export function useMitt() {
  return emitter
}
