<script setup lang="ts">
import { API_BASE_URL } from '@/config'
import { ref, watch } from 'vue'

interface UserInfo {
  UserName: string
  DisplayName: string
  Signature: string
  Introduction: string
}

const props = defineProps<{
  userInfo: UserInfo
  loading: boolean
}>()

const emit = defineEmits<{
  (e: 'info-updated'): void
}>()

// 本地表单数据
const form = ref<UserInfo>({
  UserName: '',
  DisplayName: '',
  Signature: '',
  Introduction: ''
})

const submitting = ref(false)

// 监听 props 变化，更新表单
watch(() => props.userInfo, (newVal) => {
  form.value = { ...newVal }
}, { immediate: true, deep: true })

async function handleSubmit() {
  if (submitting.value) return

  submitting.value = true

  try {
    const response = await fetch(`${API_BASE_URL}/Api/SetUserInfo`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      credentials: 'include',
      body: JSON.stringify({
        UserName: form.value.UserName,
        DisplayName: form.value.DisplayName || null,
        Signature: form.value.Signature || null,
        Introduction: form.value.Introduction || null
      })
    })

    const data = await response.json()
    if (data.Status === 'True') {
      emit('info-updated')
      alert('Profile updated successfully')
    } else {
      alert('Update failed')
    }
  } catch (error) {
    console.error('Update error:', error)
    alert('Update failed')
  } finally {
    submitting.value = false
  }
}
</script>

<template>
  <div class="AdminProfileInfoSettingDiv">
    <el-form label-width="auto" class="AdminProfileForm">
      <el-form-item label="UserName">
        <el-input v-model="form.UserName" disabled></el-input>
      </el-form-item>
      <el-form-item label="DisplayName">
        <el-input v-model="form.DisplayName" placeholder="Please enter your DisplayName"></el-input>
      </el-form-item>
      <el-form-item label="Signature">
        <el-input v-model="form.Signature" placeholder="Please enter your Signature"></el-input>
      </el-form-item>
      <el-form-item label="Introduction">
        <el-input
          v-model="form.Introduction"
          type="textarea"
          :rows="4"
          placeholder="Please enter your Introduction"
        ></el-input>
      </el-form-item>
      <el-form-item>
        <el-button
          type="primary"
          plain
          @click="handleSubmit"
          :disabled="submitting || loading"
          :loading="submitting"
          style="width: 100%"
        >
          {{ submitting ? 'Submitting...' : 'Submit' }}
        </el-button>
      </el-form-item>
    </el-form>
  </div>
</template>

<style scoped>
.AdminProfileInfoSettingDiv {
  width: 100%;
  max-width: 500px;
}

.AdminProfileForm {
  width: 100%;
}

.AdminProfileForm :deep(.el-form-item__label) {
  color: #ffffff;
}

.AdminProfileForm :deep(.el-input.is-disabled .el-input__wrapper) {
  background-color: rgba(255, 255, 255, 0.1);
  color: #cccccc;
}

.AdminProfileForm :deep(.el-textarea__inner) {
  background-color: rgba(255, 255, 255, 0.9);
  color: #000000;
}

.AdminProfileForm :deep(.el-form-item:last-child .el-form-item__content) {
  justify-content: center;
}
</style>