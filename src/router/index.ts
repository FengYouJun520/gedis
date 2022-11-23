import { App } from 'vue'
import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router'
import Layout from '@/layout/index.vue'

const routes: RouteRecordRaw[] = [
  {
    name: 'Index',
    path: '/',
    redirect: '/home',
    component: Layout,
    children: [
      {
        name: 'Home',
        path: 'home',
        component: () => import('@/views/home/index.vue'),
        meta: {
          keepalive: true,
        },
      },
      {
        name: 'Info',
        path: 'info',
        component: () => import('@/views/info/index.vue'),
        meta: {
          keepalive: true,
        },
      },
      {
        name: 'Detail',
        path: 'detail',
        component: () => import('@/views/detail/index.vue'),
      },
      {
        name: 'RedisCli',
        path: 'terminal',
        component: () => import('@/views/terminal/index.vue'),
      },
    ],
  },
]

const router = createRouter({
  routes,
  history: createWebHistory(),
})

export function setupRouter(app: App) {
  app.use(router)
}

export default router
