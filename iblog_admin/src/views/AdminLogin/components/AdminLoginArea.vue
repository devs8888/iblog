<script setup lang="ts">
import { API_BASE_URL } from '@/config'
import { ref } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()
const username = ref('')
const password = ref('')
const loading = ref(false)
const avatarUrl = ref('')

// 获取用户头像
async function fetchAvatar() {
  if (!username.value.trim()) {
    avatarUrl.value = ''
    return
  }

  try {
    const response = await fetch(`${API_BASE_URL}/Api/GetAvatarUrlByUserName`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      credentials: 'include',
      body: JSON.stringify({ UserName: username.value })
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

// 用户名输入框失去焦点时获取头像
function handleUsernameBlur() {
  fetchAvatar()
}

// SHA256 加密函数
async function sha256(message: string): Promise<string> {
  const msgBuffer = new TextEncoder().encode(message)
  const hashBuffer = await crypto.subtle.digest('SHA-256', msgBuffer)
  const hashArray = Array.from(new Uint8Array(hashBuffer))
  return hashArray.map(b => b.toString(16).padStart(2, '0')).join('')
}

async function handleSubmit() {
  if (!username.value || !password.value) {
    alert('Please enter username and password')
    return
  }

  loading.value = true

  try {
    const hashedPassword = await sha256(password.value)

    const response = await fetch(`${API_BASE_URL}/Api/Login`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      credentials: 'include',
      body: JSON.stringify({
        UserName: username.value,
        PassWord: hashedPassword,
      }),
    })

    const data = await response.json()

    if (data.Status === 'True') {
      router.push('/AdminManage')
    } else {
      alert('Invalid username or password')
    }
  } catch (error) {
    console.error('Login error:', error)
    alert('Login failed, please try again')
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="AdminLoginArea">
    <div class="LoginBox">
      <el-avatar v-if="avatarUrl" :size="100" :src="avatarUrl" class="AdminAvatar"> user </el-avatar>
      <el-avatar v-else :size="100" class="AdminAvatar"> user </el-avatar>
      <el-form label-width="auto" class="AdminLoginForm" @submit.prevent="handleSubmit">
        <el-form-item label="UserName">
          <el-input
            v-model="username"
            placeholder="Please enter your username"
            :disabled="loading"
            @blur="handleUsernameBlur"
          ></el-input>
        </el-form-item>
        <el-form-item label="Password">
          <el-input
            v-model="password"
            type="password"
            placeholder="Please enter your password"
            show-password
            :disabled="loading"
            @keyup.enter="handleSubmit"
          ></el-input>
        </el-form-item>
        <el-form-item>
          <el-button type="primary" plain @click="handleSubmit" :loading="loading" style="width: 100%">
            {{ loading ? 'Logging in...' : 'Submit' }}
          </el-button>
        </el-form-item>
      </el-form>
    </div>
  </div>
</template>

<style scoped>
.AdminLoginArea {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 100vh;
  background-color: #f0f2f5;
}

.LoginBox {
  background-color: rgb(42, 89, 138);
  border-radius: 8px;
  padding: 40px;
  display: flex;
  flex-direction: column;
  align-items: center;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}
.AdminLoginForm {
  width: 320px;
}
.AdminLoginForm :deep(.el-form-item__label) {
  color: #ffffff;
}
.AdminLoginForm :deep(.el-switch__label) {
  color: #ffffff;
}
.AdminLoginForm :deep(.el-switch__label--left) {
  color: #ffffff;
}
.AdminAvatar {
  margin-bottom: 24px;
  background-color: #1989fa;
  font-size: 24px;
}
.AdminLoginForm :deep(.el-form-item:last-child .el-form-item__content) {
  justify-content: center;
}
</style>