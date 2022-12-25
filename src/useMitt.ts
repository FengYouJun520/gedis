import mitt, { Emitter } from 'mitt'

type Events = {
  changeDb: number,
  disConnection?: void,
  fetchInfo: string,
  searchKeyTree: string,
  fetchTreeKeys: {
    id: string,
    db: number
  },
  fetchKeyInfo?: void,
  clearLogs?: void
}

const emitter: Emitter<Events> = mitt<Events>()

export function useMitt() {
  return emitter
}
