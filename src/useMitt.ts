import mitt, { Emitter } from 'mitt'

type Events = {
  changeDb: number,
  getInfo?: void,
  fetchInfo: string,
  searchKeyTree: string,
  fetchTreeKeys: {
    id: string,
    db: number
  },
  fetchKeyInfo?:void,
}

const emitter: Emitter<Events> = mitt<Events>()

export function useMitt() {
  return emitter
}
