<script setup lang="ts">
import { API_BASE_URL } from '@/config'
import { ref, onMounted } from 'vue'

const siteName = ref('')
const siteIntroduction = ref('')

const fetchSiteSetting = async () => {
  try {
    const response = await fetch(`${API_BASE_URL}/Api/GetSiteSetting`, {
      method: 'GET'
    })
    const data = await response.json()
    if (data.Status === 'True') {
      siteName.value = data.SiteName || ''
      siteIntroduction.value = data.SiteIntroduction || ''
    }
  } catch (error) {
    console.error('Failed to fetch site setting:', error)
  }
}

const submitSiteSetting = async () => {
  try {
    const response = await fetch(`${API_BASE_URL}/Api/SetSiteSetting`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        SiteName: siteName.value,
        SiteIntroduction: siteIntroduction.value
      }),
      credentials: 'include'
    })
    const data = await response.json()
    if (data.Status === 'True') {
      alert('Site setting updated successfully')
    } else {
      alert('Failed to update site setting')
    }
  } catch (error) {
    console.error('Failed to submit site setting:', error)
    alert('Failed to update site setting')
  }
}

onMounted(() => {
  fetchSiteSetting()
})
</script>

<template>
<div class="AdminWebSiteSettingAreaDiv">
  <el-form label-width="auto" class="AdminWebsiteSettingForm">
    <el-form-item label="SiteName">
      <el-input v-model="siteName" placeholder="Please enter your SiteName"></el-input>
    </el-form-item>
    <el-form-item label="Site Introduction">
      <el-input v-model="siteIntroduction" placeholder="Please enter Site Introduction"></el-input>
    </el-form-item>
  </el-form>
  <el-form-item>
    <el-button type="primary" @click="submitSiteSetting">Submit</el-button>
  </el-form-item>
</div>
</template>

<style scoped>
.AdminWebSiteSettingAreaDiv {
  color: #ffffff;
}

.SiteIconArea {
  display: flex;
  align-items: center;
  gap: 20px;
}

.SiteIconPreview {
  width: 80px;
  height: 80px;
}

:deep(.el-form-item__label) {
  color: #ffffff;
}
</style>
