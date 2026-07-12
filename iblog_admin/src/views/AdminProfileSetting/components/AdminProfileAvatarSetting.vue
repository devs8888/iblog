<script setup lang="ts">
import { API_BASE_URL } from '@/config'
import { ref } from 'vue'

const props = defineProps<{
  avatarUrl: string
  userName: string
}>()

const emit = defineEmits<{
  (e: 'avatar-uploaded'): void
}>()

const uploading = ref(false)
const fileInput = ref<HTMLInputElement | null>(null)

function triggerFileInput() {
  fileInput.value?.click()
}

async function handleFileChange(event: Event) {
  const target = event.target as HTMLInputElement
  const file = target.files?.[0]
  if (!file) return

  // 检查文件格式
  const allowedTypes = ['image/jpeg', 'image/jpg', 'image/png']
  if (!allowedTypes.includes(file.type)) {
    alert('Only JPG, JPEG, and PNG files are allowed')
    return
  }

  // 检查文件大小 (2MB)
  if (file.size > 2 * 1024 * 1024) {
    alert('File size must be less than 2MB')
    return
  }

  uploading.value = true

  try {
    const formData = new FormData()
    formData.append('avatar', file)

    const response = await fetch(`${API_BASE_URL}/Api/UploadAvatar`, {
      method: 'POST',
      credentials: 'include',
      body: formData
    })

    const data = await response.json()
    if (data.Status === 'True') {
      emit('avatar-uploaded')
    } else {
      alert('Upload failed')
    }
  } catch (error) {
    console.error('Upload error:', error)
    alert('Upload failed')
  } finally {
    uploading.value = false
    // 清空文件输入
    if (fileInput.value) {
      fileInput.value.value = ''
    }
  }
}
</script>

<template>
  <div class="AdminProfileAvatarSettingDiv">
    <el-avatar v-if="avatarUrl" :size="300" :src="avatarUrl" class="AdminAvatar"> user </el-avatar>
    <el-avatar v-else :size="300" class="AdminAvatar"> {{ userName || 'user' }} </el-avatar>
    <br><br><br>
    <input
      ref="fileInput"
      type="file"
      accept=".jpg,.jpeg,.png"
      style="display: none"
      @change="handleFileChange"
    />
    <el-button
      type="primary"
      plain
      @click="triggerFileInput"
      :disabled="uploading"
      :loading="uploading"
    >
      {{ uploading ? 'Uploading...' : 'Upload Your Avatar' }}
    </el-button>
    <p class="upload-hint">支持 JPG, PNG 格式，最大 2MB</p>
  </div>
</template>

<style scoped>
.AdminProfileAvatarSettingDiv {
  text-align: center;
  color: #ffffff;
  margin-bottom: 40px;
}

.AdminAvatar {
  font-size: 120px;
  background-color: #1989fa;
}

.upload-hint {
  margin-top: 10px;
  font-size: 12px;
  color: #cccccc;
}
</style>