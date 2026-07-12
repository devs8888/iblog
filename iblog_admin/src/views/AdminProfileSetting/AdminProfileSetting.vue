<script setup lang="ts">
import { API_BASE_URL } from '@/config'
import { ref, onMounted } from 'vue'
import AdminProfileAvatarSetting from "@/views/AdminProfileSetting/components/AdminProfileAvatarSetting.vue";
import AdminProfileInfoSetting from "@/views/AdminProfileSetting/components/AdminProfileInfoSetting.vue";

interface UserInfo {
  UserName: string
  DisplayName: string
  Signature: string
  Introduction: string
}

const userInfo = ref<UserInfo>({
  UserName: '',
  DisplayName: '',
  Signature: '',
  Introduction: ''
})

const avatarUrl = ref('')
const loading = ref(false)

async function fetchUserInfo() {
  loading.value = true
  try {
    const response = await fetch(`${API_BASE_URL}/Api/GetUserInfo`, {
      method: 'GET',
      credentials: 'include'
    })
    const data = await response.json()
    if (data.Status === 'True') {
      userInfo.value = {
        UserName: data.UserName || '',
        DisplayName: data.DisplayName || '',
        Signature: data.Signature || '',
        Introduction: data.Introduction || ''
      }
      // 获取头像URL
      if (userInfo.value.UserName) {
        fetchAvatarUrl()
      }
    }
  } catch (error) {
    console.error('Failed to fetch user info:', error)
  } finally {
    loading.value = false
  }
}

async function fetchAvatarUrl() {
  try {
    const response = await fetch(`${API_BASE_URL}/Api/GetAvatarUrlByUserName`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      credentials: 'include',
      body: JSON.stringify({ UserName: userInfo.value.UserName })
    })
    const data = await response.json()
    if (data.Status === 'True' && data.AvatarUrl) {
      avatarUrl.value = `${API_BASE_URL}${data.AvatarUrl}`
    } else {
      avatarUrl.value = ''
    }
  } catch (error) {
    console.error('Failed to fetch avatar URL:', error)
    avatarUrl.value = ''
  }
}

async function handleAvatarUploaded() {
  // 头像上传成功后刷新头像URL
  await fetchAvatarUrl()
}

async function handleInfoUpdated() {
  // 信息更新后刷新数据
  await fetchUserInfo()
}

onMounted(() => {
  fetchUserInfo()
})
</script>

<template>
  <div class="AdminProfileSettingDiv">
    <AdminProfileAvatarSetting
      :avatarUrl="avatarUrl"
      :userName="userInfo.UserName"
      @avatar-uploaded="handleAvatarUploaded"
    ></AdminProfileAvatarSetting>
    <AdminProfileInfoSetting
      :userInfo="userInfo"
      :loading="loading"
      @info-updated="handleInfoUpdated"
    ></AdminProfileInfoSetting>
  </div>
</template>

<style scoped>
.AdminProfileSettingDiv {
  background-color: rgb(42, 89, 138);
  padding: 40px;
  display: flex;
  flex-direction: column;
  align-items: center;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  min-height: 100%;
}
</style>