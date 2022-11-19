import { createApp } from 'vue'
import App from './App.vue'
import './style.css'
import 'element-plus/theme-chalk/dark/css-vars.css'
import { setupRouter } from './router'
import { setupStore } from './store'

const app = createApp(App)
setupRouter(app)
setupStore(app)
app.mount('#app')
