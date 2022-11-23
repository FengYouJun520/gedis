import { createApp } from 'vue'
import App from './App.vue'
import { setupRouter } from './router'
import { setupStore } from './store'
import 'element-plus/theme-chalk/dark/css-vars.css'
import 'uno.css'
import 'xterm/css/xterm.css'
import './style.css'
import 'xterm/lib/xterm.js'

const app = createApp(App)
setupRouter(app)
setupStore(app)
app.mount('#app')
