import { createRouter, createWebHistory, type RouteRecordRaw } from 'vue-router'


const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'containers',
    component: () => import('@/ContainerList.vue'),
  },
  {
    path: '/containers/:id',
    name: 'container',
    component: () => import('@/Container.vue'),
    meta: {
      backButton: true,
    },
  },
]

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
})

export default router
