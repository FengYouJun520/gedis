import { createApp } from 'vue'
import App from './App.vue'
import Terminal from 'vue-web-terminal'
import { setupRouter } from './router'
import { setupStore } from './store'
import 'element-plus/theme-chalk/dark/css-vars.css'
import 'uno.css'
import './style.css'

const app = createApp(App)
setupRouter(app)
setupStore(app)
app.use(Terminal)
app.mount('#app')
