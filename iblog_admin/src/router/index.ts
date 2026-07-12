import { createRouter, createWebHistory } from 'vue-router'
import { API_BASE_URL } from '@/config'

// 路由配置
const routes = [
  {
    path: "/AdminLogin",
    name: "AdminLogin",
    component: () => import('@/views/AdminLogin/AdminLogin.vue'),
    meta: { requiresAuth: false }
  },
  {
    path: "/AdminManage",
    name: "AdminManage",
    component: () => import('@/views/AdminManage/AdminManage.vue'),
    meta: { requiresAuth: true }
  },
  {
    path: "/",
    redirect: "/AdminLogin"
  }
]

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
})

// 全局导航守卫 - 权限管理
router.beforeEach(async (to, from, next) => {
  if (to.meta.requiresAuth) {
    // 需要认证，检查 Cookie 中的 Token
    try {
      const response = await fetch(`${API_BASE_URL}/Api/VerifyToken`, {
        method: 'GET',
        credentials: 'include', // 携带 Cookie
      })
      const data = await response.json()
      if (data.Status === 'True') {
        next()
      } else {
        next('/AdminLogin')
      }
    } catch {
      next('/AdminLogin')
    }
  } else {
    next()
  }
})

export default router