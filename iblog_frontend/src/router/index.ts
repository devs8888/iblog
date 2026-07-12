import { createRouter, createWebHistory } from 'vue-router'
import UserHomePage from '@/views/UserHomePage/UserHomePage.vue'
import ArticleFullPage from '@/views/ArticleFullPage/ArticleFullPage.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'UserHomePage',
      component: UserHomePage
    },
    {
      path: '/article/:id',
      name: 'ArticleFullPage',
      component: ArticleFullPage
    }
  ],
})

export default router
