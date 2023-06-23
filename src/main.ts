import { createApp } from 'vue'
import App from './App.vue'
import { setupStore } from './store'
import './userWorker'
import 'element-plus/theme-chalk/dark/css-vars.css'
import 'uno.css'
import './style.css'
import 'xterm/css/xterm.css'

const meta = document.createElement('meta')
meta.name = 'naive-ui-style'
document.head.appendChild(meta)

const app = createApp(App)
setupStore(app)
app.mount('#app')
