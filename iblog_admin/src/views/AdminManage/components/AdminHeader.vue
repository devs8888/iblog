<script setup lang="ts">
import { API_BASE_URL } from '@/config'
import { ref, computed, onMounted } from 'vue'

const userName = ref('AdminUser')
const avatarUrl = ref('')
const avatarSize = 50

const emit = defineEmits<{
  (e: 'logout'): void
}>()

const userNameStyle = computed(() => ({
  marginRight: `${userName.value.length * 10 + 20}px`
}))

async function fetchUserInfo() {
  try {
    const response = await fetch(`${API_BASE_URL}/Api/GetUserInfo`, {
      method: 'GET',
      credentials: 'include'
    })
    const data = await response.json()
    if (data.Status === 'True') {
      // 优先使用 DisplayName，如果没有则使用 UserName
      userName.value = data.DisplayName || data.UserName || 'AdminUser'
      // 获取头像
      if (data.UserName) {
        fetchAvatar(data.UserName)
      }
    }
  } catch (error) {
    console.error('Failed to fetch user info:', error)
  }
}

async function fetchAvatar(userName: string) {
  try {
    const response = await fetch(`${API_BASE_URL}/Api/GetAvatarUrlByUserName`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      credentials: 'include',
      body: JSON.stringify({ UserName: userName })
    })
    const data = await response.json()
    if (data.Status === 'True' && data.AvatarUrl) {
      avatarUrl.value = `${API_BASE_URL}${data.AvatarUrl}`
    } else {
      avatarUrl.value = ''
    }
  } catch (error) {
    console.error('Failed to fetch avatar:', error)
    avatarUrl.value = ''
  }
}

function handleLogout() {
  emit('logout')
}

onMounted(() => {
  fetchUserInfo()
})
</script>

<template>
  <div class="AdminHeader">
    <span class="AdminHeaderUserName" :style="userNameStyle">{{ userName }}</span>
    <el-button type="danger" size="small" @click="handleLogout" style="margin-right: 20px">
      Logout
    </el-button>
    <el-avatar v-if="avatarUrl" :size="avatarSize" :src="avatarUrl" class="AdminHeaderAvatar"> user </el-avatar>
    <el-avatar v-else :size="avatarSize" class="AdminHeaderAvatar"> {{ userName.charAt(0).toUpperCase() }} </el-avatar>
  </div>
</template>

<style scoped>
.AdminHeader {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  height: 100%;
  font-size: 12px;
  background-color: rgb(42, 89, 138);
  color: #ffffff;
}

.AdminHeaderAvatar {
  margin-right: 20px;
  background-color: #1989fa;
}
</style>