<script setup lang="ts">
import { API_BASE_URL } from '@/config'
import { ref, onMounted } from 'vue'

const totalPageViews = ref(0)
const articlesNumber = ref(0)
const todayPageViews = ref(0)
const todayNewArticlesNumber = ref(0)

const getTodayDate = () => {
  const now = new Date()
  const year = now.getFullYear()
  const month = String(now.getMonth() + 1).padStart(2, '0')
  const day = String(now.getDate()).padStart(2, '0')
  return `${year}-${month}-${day}`
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

const fetchTodayPageViews = async () => {
  try {
    const today = getTodayDate()
    const response = await fetch(`${API_BASE_URL}/Api/GetNewArticlesPageViewsByTime`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ Time: today })
    })
    const data = await response.json()
    if (data.Status === 'True') {
      todayPageViews.value = data.PageViews || 0
    }
  } catch (error) {
    console.error('Failed to fetch today page views:', error)
  }
}

const fetchTodayNewArticlesNumber = async () => {
  try {
    const today = getTodayDate()
    const response = await fetch(`${API_BASE_URL}/Api/GetNewArticlesNumberByTime`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ Time: today })
    })
    const data = await response.json()
    if (data.Status === 'True') {
      todayNewArticlesNumber.value = data.Number || 0
    }
  } catch (error) {
    console.error('Failed to fetch today new articles number:', error)
  }
}

onMounted(() => {
  fetchTotalPageViews()
  fetchArticlesNumber()
  fetchTodayPageViews()
  fetchTodayNewArticlesNumber()
})
</script>

<template>
<div class="AdminDashBoardCardDiv">
  <el-row :gutter="20">
    <el-col :span="6">
      <el-statistic title="Today New Articles' PageViews" :value="todayPageViews" class="AdminDashBoardCard"></el-statistic>
    </el-col>
    <el-col :span="6">
      <el-statistic title="Total PageViews" :value="totalPageViews" class="AdminDashBoardCard"></el-statistic>
    </el-col>
    <el-col :span="6">
      <el-statistic title="Articles Number" :value="articlesNumber" class="AdminDashBoardCard"></el-statistic>
    </el-col>
    <el-col :span="6">
      <el-statistic title="Today's New Articles Number" :value="todayNewArticlesNumber" class="AdminDashBoardCard"></el-statistic>
    </el-col>
  </el-row>
</div>
</template>

<style scoped>
.AdminDashBoardCardDiv {
  background-color: rgb(42, 89, 138);
  padding: 30px;
  text-align: center;
}

.AdminDashBoardCard {
  height: 120px;
  display: flex;
  flex-direction: column;
  justify-content: center;
  background-color: rgba(255, 255, 255, 0.15);
  border-radius: 8px;
}

:deep(.el-statistic__head) {
  color: #ffffff;
  font-size: 16px;
}

:deep(.el-statistic__content) {
  color: #ffffff;
  font-size: 28px;
  font-weight: bold;
}
</style>
