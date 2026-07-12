<script setup lang="ts">
import { API_BASE_URL } from '@/config'
import { ref, onMounted, onBeforeUnmount } from 'vue'
import { useRouter } from 'vue-router'
import AdminSideBar from "@/views/AdminManage/components/AdminSideBar.vue";
import AdminHeader from "@/views/AdminManage/components/AdminHeader.vue";
import AdminDashBoard from "@/views/AdminDashBoard/AdminDashBoard.vue";
import AdminProfileSetting from "@/views/AdminProfileSetting/AdminProfileSetting.vue";
import AdminSecuritySetting from "@/views/AdminSecuritySetting/AdminSecuritySetting.vue";
import AdminWebsiteSetting from "@/views/AdminWebsiteSetting/AdminWebsiteSetting.vue";
import AdminArticleManage from "@/views/AdminArticleManage/AdminArticleManage.vue";

const router = useRouter()
const currentView = ref('dashboard')

// 心跳配置
const HEARTBEAT_INTERVAL = 60000 // 每分钟检查一次

// 发送心跳到后端（从 Cookie 自动携带）
async function sendHeartbeat(): Promise<boolean> {
  try {
    const response = await fetch(`${API_BASE_URL}/Api/Heartbeat`, {
      method: 'POST',
      credentials: 'include', // 携带 Cookie
    })
    const data = await response.json()
    return data.Status === 'True'
  } catch {
    return false
  }
}

// 验证 Token（从 Cookie 自动携带）
async function verifyToken(): Promise<boolean> {
  try {
    const response = await fetch(`${API_BASE_URL}/Api/VerifyToken`, {
      method: 'GET',
      credentials: 'include', // 携带 Cookie
    })
    const data = await response.json()
    return data.Status === 'True'
  } catch {
    return false
  }
}

let heartbeatTimer: number | null = null

// 启动心跳
function startHeartbeat(): void {
  if (heartbeatTimer !== null) return

  heartbeatTimer = window.setInterval(async () => {
    const isValid = await sendHeartbeat()
    if (!isValid) {
      stopHeartbeat()
      router.push('/AdminLogin')
    }
  }, HEARTBEAT_INTERVAL)
}

// 停止心跳
function stopHeartbeat(): void {
  if (heartbeatTimer !== null) {
    clearInterval(heartbeatTimer)
    heartbeatTimer = null
  }
}

// 登出 - 调用后端清空 Cookie
async function handleLogout(): Promise<void> {
  try {
    await fetch(`${API_BASE_URL}/Api/LogOut`, {
      method: 'POST',
      credentials: 'include',
    })
  } catch (error) {
    console.error('Logout error:', error)
  } finally {
    stopHeartbeat()
    router.push('/AdminLogin')
  }
}

const handleMenuSelect = (view: string) => {
  currentView.value = view
}

// 页面加载时验证权限
onMounted(async () => {
  const isValid = await verifyToken()
  if (!isValid) {
    router.push('/AdminLogin')
    return
  }
  startHeartbeat()
})

onBeforeUnmount(() => {
  stopHeartbeat()
})
</script>

<template>
  <el-container>
    <el-aside width="200px">
      <div class="AdminSideBarDiv"><AdminSideBar @menu-select="handleMenuSelect"></AdminSideBar></div>
    </el-aside>
    <el-container>
      <el-header>
        <div class="AdminHeaderDiv"><AdminHeader @logout="handleLogout"></AdminHeader></div>
      </el-header>
      <el-main>
        <AdminDashBoard v-if="currentView === 'dashboard'"></AdminDashBoard>
        <AdminProfileSetting v-if="currentView === 'profile'"></AdminProfileSetting>
        <AdminSecuritySetting v-if="currentView === 'security'"></AdminSecuritySetting>
        <AdminWebsiteSetting v-if="currentView === 'sitesetting'"></AdminWebsiteSetting>
        <AdminArticleManage v-if="currentView === 'articlemanage'"></AdminArticleManage>
      </el-main>
    </el-container>
  </el-container>
</template>

<style scoped>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

.el-container {
  height: 100vh;
  width: 100vw;
}

.el-aside {
  height: 100%;
}

.el-header {
  padding: 0;
  margin: 0;
}

.AdminSideBarDiv {
  height: 100%;
  background-color: rgb(42, 89, 138);
  width: 200px;
}

.AdminHeaderDiv {
  width: 100%;
  background-color: rgb(42, 89, 138);
}
</style>
