import { createApp } from 'vue'
import { createPinia } from 'pinia'
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import App from './App.vue'
import router from './router'
import { API_BASE_URL } from './config'

const app = createApp(App)
app.use(ElementPlus)
app.use(createPinia())
app.use(router)

const fetchSiteSetting = async () => {
  try {
    const response = await fetch(`${API_BASE_URL}/Api/GetSiteSetting`, {
      method: 'GET'
    })
    const data = await response.json()
    if (data.Status === 'True') {
      const siteName = data.SiteName || ''
      const siteIntroduction = data.SiteIntroduction || ''
      document.title = `${siteName}|${siteIntroduction}`
    }
  } catch (error) {
    console.error('Failed to fetch site setting:', error)
  }
}

app.mount('#app')
fetchSiteSetting()
