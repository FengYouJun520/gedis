import { createApp } from 'vue'
import App from './App.vue'
import { setupStore } from './store'
import 'element-plus/theme-chalk/dark/css-vars.css'
import 'uno.css'
import './style.css'

const app = createApp(App)
setupStore(app)
app.mount('#app')
