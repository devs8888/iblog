<script setup lang="ts">
import { API_BASE_URL } from '@/config'
import { ref } from 'vue'
import { ElMessage } from 'element-plus'
import { useRouter } from 'vue-router'

const router = useRouter()
const password = ref('')
const loading = ref(false)

const handleSubmit = async () => {
  if (!password.value) {
    ElMessage.error('Please enter your password')
    return
  }

  loading.value = true
  try {
    const response = await fetch(`${API_BASE_URL}/Api/ChangePassWord`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      credentials: 'include',
      body: JSON.stringify({ NewPassWord: password.value })
    })
    const data = await response.json()
    if (data.Status === 'True') {
      ElMessage.success('Password changed successfully')
      password.value = ''
      // 登出并返回登录界面
      await fetch(`${API_BASE_URL}/Api/LogOut`, {
        method: 'POST',
        credentials: 'include'
      })
      router.push('/AdminLogin')
    } else {
      ElMessage.error('Failed to change password')
    }
  } catch {
    ElMessage.error('Failed to change password')
  } finally {
    loading.value = false
  }
}
</script>

<template>
<div class="AdminSecurityAccountSettingDiv">
  <el-form label-width="auto" class="AdminSecurityForm">
    <el-form-item label="PassWord">
      <div class="password-row">
        <el-input
          v-model="password"
          placeholder="Please enter your new PassWord"
          type="password"
          show-password
          @keyup.enter="handleSubmit"
        ></el-input>
        <el-button type="primary" plain @click="handleSubmit" :loading="loading">Submit</el-button>
      </div>
    </el-form-item>
  </el-form>
</div>
</template>

<style scoped>
.AdminSecurityAccountSettingDiv {
  color: #ffffff;
}

:deep(.el-form-item__label) {
  color: #ffffff;
}

.password-row {
  display: flex;
  gap: 12px;
  width: 100%;
}
</style>
