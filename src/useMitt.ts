import mitt, { Emitter } from 'mitt'

type Events = {
  changeDb: number,
  getInfo?: string,
  fetchInfo: string,
  fetchTreeKeys: {
    id: string,
    db: number
  }
}

const emitter: Emitter<Events> = mitt<Events>()

export function useMitt() {
  return emitter
}
