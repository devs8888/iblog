<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { API_BASE_URL } from '@/config'

const userInfo = ref({
  DisplayName: '',
  Signature: '',
  Introduction: ''
})
const avatarUrl = ref('')
const totalPageViews = ref(0)
const articlesNumber = ref(0)

const fetchShowUserInfo = async () => {
  try {
    const response = await fetch(`${API_BASE_URL}/Api/GetShowUserInfo`, {
      method: 'GET'
    })
    const data = await response.json()
    if (data.Status === 'True') {
      userInfo.value = {
        DisplayName: data.DisplayName || '',
        Signature: data.Signature || '',
        Introduction: data.Introduction || ''
      }
      if (data.AvatarUrl) {
        // 提取头像文件名
        const url = new URL(data.AvatarUrl, `${API_BASE_URL}`)
        const avatarName = url.searchParams.get('AvatarName')
        if (avatarName) {
          avatarUrl.value = `${API_BASE_URL}/Api/GetAvatar?AvatarName=${avatarName}`
        }
      }
    }
  } catch (error) {
    console.error('Failed to fetch user info:', error)
  }
}

const fetchTotalPageViews = async () => {
  try {
    const response = await fetch(`${API_BASE_URL}/Api/GetTotalPageViews`, {
      method: 'GET'
    })
    const data = await response.json()
    if (data.Status === 'True') {
      totalPageViews.value = data.PageViews || 0
    }
  } catch (error) {
    console.error('Failed to fetch total page views:', error)
  }
}

const fetchArticlesNumber = async () => {
  try {
    const response = await fetch(`${API_BASE_URL}/Api/GetArticleNumber`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ KeyWord: null })
    })
    const data = await response.json()
    if (data.Status === 'True') {
      articlesNumber.value = data.Number || 0
    }
  } catch (error) {
    console.error('Failed to fetch articles number:', error)
  }
}

onMounted(() => {
  fetchShowUserInfo()
  fetchTotalPageViews()
  fetchArticlesNumber()
})
</script>

<template>
  <div class="UserHomePageSideBarDiv">
    <el-avatar v-if="avatarUrl" :src="avatarUrl" class="UserAvatar"> Admin </el-avatar>
    <el-avatar v-else class="UserAvatar"> Admin </el-avatar>
    <el-card style="max-width: 300px" class="AdminInfoCard">
      <template #header>
        <div class="AdminInfoHeader">
          <span>Admin User Info</span>
        </div>
      </template>
      <p>Name: {{ userInfo.DisplayName || 'N/A' }}</p>
      <p>Signature: {{ userInfo.Signature || 'N/A' }}</p>
      <p>Introdution: {{ userInfo.Introduction || 'N/A' }}</p>
    </el-card>
    <el-card style="max-width: 300px" class="SiteInfoCard">
      <p>Articles Number: {{ articlesNumber }}</p>
      <p>Total PageViews: {{ totalPageViews }}</p>
    </el-card>
  </div>
</template>

<style scoped>
.UserHomePageSideBarDiv {
  width: 100%;
  height: 100%;
  background-color: rgb(42, 89, 138);
  display: flex;
  flex-direction: column;
  gap: 20px;
  padding: 20px;
}
.UserAvatar{
  text-align: center;
  width: 150px;
  height: 150px;
  font-size: 48px;
  line-height: 150px;
  margin: 0 auto;
}
:deep(.el-card) {
  background-color: rgba(255, 255, 255, 0.15);
  color: #ffffff;
}

:deep(.el-card__header) {
  background-color: transparent;
  color: #ffffff;
}

:deep(.el-avatar) {
  background-color: rgba(255, 255, 255, 0.2);
  color: #ffffff;
}
</style>
