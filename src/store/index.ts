import PiniaPersisted from 'pinia-plugin-persistedstate'
import { App } from 'vue'

const pinia = createPinia()
pinia.use(PiniaPersisted)

export function setupStore(app: App) {
  app.use(pinia)
}

export default pinia
