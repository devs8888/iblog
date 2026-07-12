<script setup lang="ts">
import { API_BASE_URL } from '@/config'
import { ref, onMounted } from 'vue'

interface Article {
  Id: string
  Sender: string
  Version: number
  Date: string
  Title: string
  Content: string
  PageViews: number
  File: string | null
}

let articleData = ref<Article[]>([])
const currentPage = ref(1)
const totalArticles = ref(0)
const pageSize = 3
const dialogVisible = ref(false)
const fileListDialogVisible = ref(false)
const currentArticleFiles = ref<string[]>([])
const currentArticleId = ref('')
const currentFileListVersion = ref(0)
const deleteDialogVisible = ref(false)
const deleteTargetId = ref('')
const deleteVersions = ref<number[]>([])
const deleteVersionOptions = ref<{label: string, value: number}[]>([])
const changeDialogVisible = ref(false)
const changeForm = ref({
  Id: '',
  Title: '',
  Content: ''
})
const changeCurrentVersion = ref(0) // 当前编辑的文章版本
const changeFileList = ref<string[]>([])
const changeNewFiles = ref<FileList | null>(null)
const changeDeletedFiles = ref<string[]>([]) // 被删除的文件列表
const changeFileInput = ref<HTMLInputElement | null>(null)
const changeSubmitting = ref(false)
const changeLoading = ref(false)

const versionDialogVisible = ref(false)
const versionData = ref<any[]>([])
const versionTargetId = ref('')
const versionDetailDialogVisible = ref(false)
const versionDetailData = ref<any>(null)
const versionDetailFiles = ref<string[]>([])
const versionDetailLoading = ref(false)

const articleForm = ref({
  Title: '',
  Content: ''
})
const submitting = ref(false)
const fileInput = ref<HTMLInputElement | null>(null)
const loading = ref(false)
const deleteLoading = ref(false)

function showAddDialog() {
  dialogVisible.value = true
}

function showDeleteConfirm(articleId: string) {
  deleteTargetId.value = articleId
  deleteVersions.value = []
  fetchVersionListForDelete(articleId)
  deleteDialogVisible.value = true
}

async function fetchVersionListForDelete(articleId: string) {
  try {
    const response = await fetch(`${API_BASE_URL}/Api/GetArticleVersion`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      credentials: 'include',
      body: JSON.stringify({ Id: articleId })
    })
    const data = await response.json()
    if (data.Status === 'True' && data.ArticleInfo) {
      deleteVersionOptions.value = data.ArticleInfo.map((v: any) => ({
        label: `Version ${v.Version} - ${v.Title}`,
        value: v.Version
      }))
    }
  } catch (error) {
    console.error('Failed to fetch version list:', error)
  }
}

async function handleDelete() {
  if (!deleteTargetId.value || deleteVersions.value.length === 0) return

  deleteLoading.value = true
  try {
    for (const version of deleteVersions.value) {
      const response = await fetch(`${API_BASE_URL}/Api/DeleteArticle`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        credentials: 'include',
        body: JSON.stringify({ Id: deleteTargetId.value, Version: version })
      })
      const data = await response.json()
      if (data.Status !== 'True') {
        alert(`Failed to delete version ${version}`)
        return
      }
    }
    deleteDialogVisible.value = false
    deleteTargetId.value = ''
    deleteVersions.value = []
    await fetchArticleNumber()
    await fetchArticleInfo(currentPage.value)
  } catch (error) {
    console.error('Delete error:', error)
    alert('Failed to delete article')
  } finally {
    deleteLoading.value = false
  }
}

function closeDialog() {
  dialogVisible.value = false
  articleForm.value = {
    Title: '',
    Content: ''
  }
  fileList.value = null
  if (fileInput.value) {
    fileInput.value.value = ''
  }
}

function closeFileListDialog() {
  fileListDialogVisible.value = false
  currentArticleFiles.value = []
  currentArticleId.value = ''
}

function showFileList(articleId: string, version: number) {
  currentArticleId.value = articleId
  currentFileListVersion.value = version
  fetchFileList(articleId, version)
  fileListDialogVisible.value = true
}

async function showChangeDialog(article: Article) {
  changeForm.value = {
    Id: article.Id,
    Title: article.Title,
    Content: article.Content
  }
  changeCurrentVersion.value = article.Version
  changeDialogVisible.value = true
  // 获取完整内容
  try {
    const response = await fetch(`${API_BASE_URL}/Api/GetArticleFull`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      credentials: 'include',
      body: JSON.stringify({ Id: article.Id, Version: article.Version })
    })
    const data = await response.json()
    if (data.Status === 'True' && data.Article) {
      changeForm.value.Title = data.Article.Title
      changeForm.value.Content = data.Article.Content
    }
  } catch (error) {
    console.error('Failed to fetch full article:', error)
  }
  // 获取文件列表（带Version参数）
  fetchChangeFileList(article.Id, article.Version)
}

async function fetchChangeFileList(articleId: string, version: number) {
  changeLoading.value = true
  try {
    const response = await fetch(`${API_BASE_URL}/Api/GetArticleFile?Id=${articleId}&Version=${version}`)
    const data = await response.json()
    if (data.Status === 'True') {
      const files: string[] = []
      for (let i = 1; i <= 10; i++) {
        const key = `File${i}`
        if (data[key]) {
          files.push(data[key])
        }
      }
      changeFileList.value = files
    } else {
      changeFileList.value = []
    }
  } catch (error) {
    console.error('Failed to fetch file list:', error)
    changeFileList.value = []
  } finally {
    changeLoading.value = false
  }
}

function removeFileFromList(filename: string) {
  changeFileList.value = changeFileList.value.filter(f => f !== filename)
  // 跟踪被删除的文件
  if (!changeDeletedFiles.value.includes(filename)) {
    changeDeletedFiles.value.push(filename)
  }
}

function triggerChangeFileInput() {
  changeFileInput.value?.click()
}

function handleChangeFileChange(event: Event) {
  const target = event.target as HTMLInputElement
  if (target.files && target.files.length > 0) {
    changeNewFiles.value = target.files
    // 构建新文件列表用于显示
    const newFiles: string[] = []
    for (let i = 0; i < target.files.length; i++) {
      newFiles.push(target.files[i].name)
    }
    // 合并现有文件和_newFiles（去重）
    const merged = [...changeFileList.value, ...newFiles.filter(f => !changeFileList.value.includes(f))]
    changeFileList.value = merged
    // 如果添加的文件之前被删除，从已删除列表中移除
    changeDeletedFiles.value = changeDeletedFiles.value.filter(f => !newFiles.includes(f))
  }
}

async function handleChangeSubmit() {
  if (!changeForm.value.Title.trim() || !changeForm.value.Content.trim()) {
    alert('Please fill in all fields')
    return
  }

  changeSubmitting.value = true
  try {
    // 获取当前用户
    const sender = await getCurrentUserName()
    if (!sender) {
      alert('Failed to get current user')
      return
    }

    // 调用修改接口
    const response = await fetch(`${API_BASE_URL}/Api/AdminChangeArticle`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      credentials: 'include',
      body: JSON.stringify({
        Id: changeForm.value.Id,
        Sender: sender,
        Title: changeForm.value.Title,
        Content: changeForm.value.Content,
        DeletedFiles: changeDeletedFiles.value
      })
    })
    const data = await response.json()
    if (data.Status === 'True') {
      // 上传新文件（使用返回的新版本号）
      if (changeNewFiles.value && changeNewFiles.value.length > 0) {
        for (let i = 0; i < changeNewFiles.value.length; i++) {
          const file = changeNewFiles.value[i]
          const formData = new FormData()
          formData.append('Id', changeForm.value.Id)
          formData.append('Version', data.Version.toString())
          formData.append('file', file)
          await fetch(`${API_BASE_URL}/Api/UploadArticleFile`, {
            method: 'POST',
            credentials: 'include',
            body: formData
          })
        }
        // 更新文件列表（使用新版本号）
        await fetch(`${API_BASE_URL}/Api/UpdateArticleFile`, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          credentials: 'include',
          body: JSON.stringify({
            ArticleId: changeForm.value.Id,
            FilePath: `/Api/GetArticleFile?Id=${changeForm.value.Id}&Version=${data.Version}`
          })
        })
      }
      alert('Article updated successfully')
      closeChangeDialog()
      await fetchArticleNumber()
      await fetchArticleInfo(currentPage.value)
    } else {
      alert('Failed to update article')
    }
  } catch (error) {
    console.error('Change error:', error)
    alert('Failed to update article')
  } finally {
    changeSubmitting.value = false
  }
}

function closeChangeDialog() {
  changeDialogVisible.value = false
  changeForm.value = { Id: '', Title: '', Content: '' }
  changeCurrentVersion.value = 0
  changeFileList.value = []
  changeNewFiles.value = null
  changeDeletedFiles.value = []
  if (changeFileInput.value) {
    changeFileInput.value.value = ''
  }
}

function showVersionDialog(articleId: string) {
  versionTargetId.value = articleId
  fetchVersionList(articleId)
  versionDialogVisible.value = true
}

async function fetchVersionList(articleId: string) {
  versionData.value = []
  try {
    const response = await fetch(`${API_BASE_URL}/Api/GetArticleVersion`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ Id: articleId })
    })
    const data = await response.json()
    if (data.Status === 'True' && data.ArticleInfo) {
      versionData.value = data.ArticleInfo
    }
  } catch (error) {
    console.error('Failed to fetch version list:', error)
  }
}

function closeVersionDialog() {
  versionDialogVisible.value = false
  versionData.value = []
  versionTargetId.value = ''
}

async function showVersionDetail(version: any) {
  versionDetailLoading.value = true
  versionDetailData.value = null
  try {
    const fullResponse = await fetch(`${API_BASE_URL}/Api/GetArticleFull`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ Id: versionTargetId.value, Version: version.Version })
    })
    const fullData = await fullResponse.json()
    if (fullData.Status === 'True' && fullData.Article) {
      versionDetailData.value = fullData.Article
    }
    const fileResponse = await fetch(`${API_BASE_URL}/Api/GetArticleFile?Id=${versionTargetId.value}&Version=${version.Version}`)
    const fileData = await fileResponse.json()
    if (fileData.Status === 'True') {
      const files: string[] = []
      for (let i = 1; i <= 10; i++) {
        const key = `File${i}`
        if (fileData[key]) {
          files.push(fileData[key])
        }
      }
      versionDetailFiles.value = files
    } else {
      versionDetailFiles.value = []
    }
    versionDetailDialogVisible.value = true
  } catch (error) {
    console.error('Failed to fetch version detail:', error)
  } finally {
    versionDetailLoading.value = false
  }
}

function closeVersionDetailDialog() {
  versionDetailDialogVisible.value = false
  versionDetailData.value = null
  versionDetailFiles.value = []
}

async function fetchFileList(articleId: string, version: number) {
  try {
    const response = await fetch(`${API_BASE_URL}/Api/GetArticleFile?Id=${articleId}&Version=${version}`)
    const data = await response.json()
    if (data.Status === 'True') {
      const files: string[] = []
      for (let i = 1; i <= 10; i++) {
        const key = `File${i}`
        if (data[key]) {
          files.push(data[key])
        }
      }
      currentArticleFiles.value = files
    } else {
      currentArticleFiles.value = []
    }
  } catch (error) {
    console.error('Failed to fetch file list:', error)
    currentArticleFiles.value = []
  }
}

const fileList = ref<FileList | null>(null)

function triggerFileInput() {
  fileInput.value?.click()
}

function handleFileChange(event: Event) {
  const target = event.target as HTMLInputElement
  fileList.value = target.files
}

async function getCurrentUserName(): Promise<string> {
  try {
    const response = await fetch(`${API_BASE_URL}/Api/GetUserInfo`, {
      method: 'GET',
      credentials: 'include'
    })
    const data = await response.json()
    if (data.Status === 'True' && data.UserName) {
      return data.UserName
    }
  } catch (error) {
    console.error('Failed to get user info:', error)
  }
  return ''
}

async function fetchArticleNumber() {
  try {
    const response = await fetch(`${API_BASE_URL}/Api/GetArticleNumber`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      credentials: 'include',
      body: JSON.stringify({ KeyWord: null })
    })
    const data = await response.json()
    if (data.Status === 'True') {
      totalArticles.value = data.Number || 0
    }
  } catch (error) {
    console.error('Failed to fetch article number:', error)
  }
}

async function fetchArticleInfo(page: number) {
  loading.value = true
  try {
    const response = await fetch(`${API_BASE_URL}/Api/GetArticleInfo`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      credentials: 'include',
      body: JSON.stringify({ Page: page })
    })
    const data = await response.json()
    if (data.Status === 'True' && data.ArticleInfo) {
      articleData.value = data.ArticleInfo
    } else {
      articleData.value = []
    }
  } catch (error) {
    console.error('Failed to fetch article info:', error)
    articleData.value = []
  } finally {
    loading.value = false
  }
}

function handlePageChange(page: number) {
  currentPage.value = page
  fetchArticleInfo(page)
}

async function handleSubmit() {
  if (!articleForm.value.Title.trim() || !articleForm.value.Content.trim()) {
    alert('Please fill in all fields')
    return
  }

  submitting.value = true

  try {
    const sender = await getCurrentUserName()
    if (!sender) {
      alert('Failed to get current user')
      return
    }

    const response = await fetch(`${API_BASE_URL}/Api/AdminAddArticle`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      credentials: 'include',
      body: JSON.stringify({
        Sender: sender,
        Title: articleForm.value.Title,
        Content: articleForm.value.Content
      })
    })

    const data = await response.json()
    if (data.Status === 'True') {
      const articleId = data.ArticleId || data.Id

      if (fileList.value && fileList.value.length > 0 && articleId) {
        const uploadSuccess = await uploadFiles(articleId)
        if (!uploadSuccess) {
          alert('Article created but file upload failed')
          return
        }
      }

      alert('Article submitted successfully')
      closeDialog()
      await fetchArticleNumber()
      await fetchArticleInfo(1)
      currentPage.value = 1
    } else {
      alert('Failed to submit article')
    }
  } catch (error) {
    console.error('Submit error:', error)
    alert('Failed to submit article')
  } finally {
    submitting.value = false
  }
}

async function uploadFiles(articleId: string): Promise<boolean> {
  if (!fileList.value || fileList.value.length === 0) {
    return true
  }

  try {
    for (let i = 0; i < fileList.value.length; i++) {
      const file = fileList.value[i]
      const formData = new FormData()
      formData.append('Id', articleId)
      formData.append('Version', '1')
      formData.append('file', file)

      const response = await fetch(`${API_BASE_URL}/Api/UploadArticleFile`, {
        method: 'POST',
        credentials: 'include',
        body: formData
      })

      const data = await response.json()
      if (data.Status !== 'True') {
        console.error('File upload failed:', file.name)
        return false
      }
    }

    await fetch(`${API_BASE_URL}/Api/UpdateArticleFile`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      credentials: 'include',
      body: JSON.stringify({
        ArticleId: articleId,
        FilePath: `/Api/GetArticleFile?Id=${articleId}&Version=1`
      })
    })

    return true
  } catch (error) {
    console.error('Upload files error:', error)
    return false
  }
}

onMounted(async () => {
  await fetchArticleNumber()
  await fetchArticleInfo(1)
})
</script>

<template>
  <div class="AdminArticleManageAreaDiv">
    <el-table :data="articleData" v-loading="loading" class="article-table">
      <el-table-column prop="Id" label="Id"></el-table-column>
      <el-table-column prop="Sender" label="Sender"></el-table-column>
      <el-table-column prop="Version" label="Version"></el-table-column>
      <el-table-column prop="Date" label="Date"></el-table-column>
      <el-table-column prop="Title" label="Title"></el-table-column>
      <el-table-column prop="Content" label="Content"></el-table-column>
      <el-table-column prop="PageViews" label="PageViews"></el-table-column>
      <el-table-column label="File">
        <template #default="scope">
          <el-button
            v-if="scope.row.File"
            type="primary"
            plain
            size="small"
            @click="showFileList(scope.row.Id, scope.row.Version)"
          >
            File List
          </el-button>
          <span v-else>Null</span>
        </template>
      </el-table-column>
      <el-table-column label="Action">
        <template #default="scope">
          <el-button type="danger" plain size="small" @click="showDeleteConfirm(scope.row.Id)">Delete</el-button>
          <el-button type="primary" plain size="small" @click="showChangeDialog(scope.row)">Change</el-button>
          <el-button type="primary" plain size="small" @click="showVersionDialog(scope.row.Id)">Show All Version</el-button>
        </template>
      </el-table-column>
    </el-table>

    <el-pagination
      v-if="totalArticles > 0"
      :current-page="currentPage"
      :page-size="pageSize"
      :total="totalArticles"
      layout="prev, pager, next"
      @current-change="handlePageChange"
      class="pagination"
    ></el-pagination>

    <el-button type="primary" plain @click="showAddDialog" style="margin-top: 20px">Add Article</el-button>

    <!-- 添加文章对话框 -->
    <el-dialog
      v-model="dialogVisible"
      title="Add Article"
      width="500px"
      :close-on-click-modal="false"
      @close="closeDialog"
      class="article-dialog"
    >
      <el-form :model="articleForm" label-width="auto" class="article-form">
        <el-form-item label="Title">
          <el-input
            v-model="articleForm.Title"
            placeholder="Please Input Your Title"
          ></el-input>
        </el-form-item>
        <el-form-item label="Content">
          <el-input
            v-model="articleForm.Content"
            type="textarea"
            :rows="6"
            placeholder="Please Input Your Content"
          ></el-input>
        </el-form-item>
        <el-form-item label="Files">
          <input
            ref="fileInput"
            type="file"
            multiple
            style="display: none"
            @change="handleFileChange"
          />
          <el-button type="primary" plain @click="triggerFileInput">
            Select Files
          </el-button>
          <div v-if="fileList && fileList.length > 0" style="margin-top: 10px; color: #ffffff;">
            Selected: {{ fileList.length }} file(s)
          </div>
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="closeDialog">Cancel</el-button>
        <el-button type="primary" @click="handleSubmit" :loading="submitting">
          {{ submitting ? 'Submitting...' : 'Submit' }}
        </el-button>
      </template>
    </el-dialog>

    <!-- 文件列表对话框 -->
    <el-dialog
      v-model="fileListDialogVisible"
      title="File List"
      width="400px"
      :close-on-click-modal="false"
      @close="closeFileListDialog"
      class="article-dialog"
    >
      <div class="file-list">
        <div v-if="currentArticleFiles.length > 0" style="display: flex; flex-direction: column; gap: 8px;">
          <a
            v-for="file in currentArticleFiles"
            :key="file"
            :href="`${API_BASE_URL}/Api/GetArticleFile?Id=${currentArticleId}&Version=${currentFileListVersion}&FileName=${file}`"
            target="_blank"
            class="file-link"
          >
            {{ file }}
          </a>
        </div>
        <div v-else style="color: #ffffff;">No files</div>
      </div>
      <template #footer>
        <el-button @click="closeFileListDialog">Close</el-button>
      </template>
    </el-dialog>

    <!-- 删除确认对话框 -->
    <el-dialog
      v-model="deleteDialogVisible"
      title="Delete Article"
      width="500px"
      :close-on-click-modal="false"
      class="article-dialog"
    >
      <div style="color: #ffffff; margin-bottom: 15px;">Select versions to delete:</div>
      <el-checkbox-group v-model="deleteVersions">
        <el-checkbox v-for="option in deleteVersionOptions" :key="option.value" :label="option.value">
          {{ option.label }}
        </el-checkbox>
      </el-checkbox-group>
      <template #footer>
        <el-button @click="deleteDialogVisible = false">Cancel</el-button>
        <el-button type="danger" @click="handleDelete" :loading="deleteLoading" :disabled="deleteVersions.length === 0">Delete</el-button>
      </template>
    </el-dialog>

    <!-- 修改文章对话框 -->
    <el-dialog
      v-model="changeDialogVisible"
      title="Change Article"
      width="500px"
      :close-on-click-modal="false"
      @close="closeChangeDialog"
      class="article-dialog"
    >
      <el-form :model="changeForm" label-width="auto" class="article-form">
        <el-form-item label="Title">
          <el-input
            v-model="changeForm.Title"
            placeholder="Please Input Your Title"
          ></el-input>
        </el-form-item>
        <el-form-item label="Content">
          <el-input
            v-model="changeForm.Content"
            type="textarea"
            :rows="6"
            placeholder="Please Input Your Content"
          ></el-input>
        </el-form-item>
        <el-form-item label="Current Files">
          <div v-if="changeFileList.length > 0" class="file-list">
            <div v-for="file in changeFileList" :key="file" class="file-item">
              <span>{{ file }}</span>
              <el-button type="danger" size="small" @click="removeFileFromList(file)">Remove</el-button>
            </div>
          </div>
          <div v-else style="color: #cccccc;">No files</div>
        </el-form-item>
        <el-form-item label="Upload New Files">
          <input
            ref="changeFileInput"
            type="file"
            multiple
            style="display: none"
            @change="handleChangeFileChange"
          />
          <el-button type="primary" plain @click="triggerChangeFileInput">
            Select Files
          </el-button>
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="closeChangeDialog">Cancel</el-button>
        <el-button type="primary" @click="handleChangeSubmit" :loading="changeSubmitting">
          {{ changeSubmitting ? 'Submitting...' : 'Submit' }}
        </el-button>
      </template>
    </el-dialog>

    <!-- 版本列表对话框 -->
    <el-dialog
      v-model="versionDialogVisible"
      title="All Versions"
      width="700px"
      :close-on-click-modal="false"
      @close="closeVersionDialog"
      class="article-dialog"
    >
      <el-table :data="versionData">
        <el-table-column prop="Version" label="Version" width="100"></el-table-column>
        <el-table-column prop="Date" label="Date" width="150"></el-table-column>
        <el-table-column prop="Title" label="Title"></el-table-column>
        <el-table-column prop="Content" label="Content"></el-table-column>
        <el-table-column prop="PageViews" label="PageViews" width="100"></el-table-column>
        <el-table-column label="Action" width="120">
          <template #default="scope">
            <el-button type="primary" plain size="small" @click="showVersionDetail(scope.row)">Show All Info</el-button>
          </template>
        </el-table-column>
      </el-table>
      <template #footer>
        <el-button @click="closeVersionDialog">Close</el-button>
      </template>
    </el-dialog>

    <!-- 版本详情对话框 -->
    <el-dialog
      v-model="versionDetailDialogVisible"
      title="Version Detail"
      width="500px"
      :close-on-click-modal="false"
      @close="closeVersionDetailDialog"
      class="article-dialog"
    >
      <div v-if="versionDetailData" v-loading="versionDetailLoading">
        <el-form label-width="auto" class="detail-form">
          <el-form-item label="Version">{{ versionDetailData.Version }}</el-form-item>
          <el-form-item label="Title">{{ versionDetailData.Title }}</el-form-item>
          <el-form-item label="Content">
            <div style="white-space: pre-wrap; color: #ffffff;">{{ versionDetailData.Content }}</div>
          </el-form-item>
          <el-form-item label="Files">
            <div v-if="versionDetailFiles.length > 0" style="display: flex; flex-direction: column; gap: 8px;">
              <a
                v-for="file in versionDetailFiles"
                :key="file"
                :href="`${API_BASE_URL}/Api/GetArticleFile?Id=${versionTargetId}&Version=${versionDetailData.Version}&FileName=${file}`"
                target="_blank"
                class="file-link"
              >
                {{ file }}
              </a>
            </div>
            <span v-else style="color: #cccccc;">No files</span>
          </el-form-item>
        </el-form>
      </div>
      <template #footer>
        <el-button @click="closeVersionDetailDialog">Close</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<style scoped>
.AdminArticleManageAreaDiv {
  color: #ffffff;
  width: 100%;
}

.article-table {
  width: 100%;
}

.pagination {
  margin-top: 20px;
  justify-content: center;
}

:deep(.el-table) {
  background-color: rgb(42, 89, 138);
  color: #ffffff;
}

:deep(.el-table__header-wrapper th) {
  background-color: rgb(42, 89, 138);
  color: #ffffff;
}

:deep(.el-table__fixed-header-wrapper th) {
  background-color: rgb(42, 89, 138) !important;
  color: #ffffff !important;
}

:deep(.el-table__fixed-right-header .el-table__header-wrapper th),
:deep(.el-table__fixed-left-header .el-table__header-wrapper th) {
  background-color: rgb(42, 89, 138) !important;
  color: #ffffff !important;
}

:deep(.el-table__body-wrapper tr) {
  background-color: rgb(42, 89, 138);
  color: #ffffff;
}

:deep(.el-table__row:hover td) {
  background-color: rgba(255, 255, 255, 0.1);
}

:deep(.el-table__row td) {
  pointer-events: auto;
}

:deep(.el-table__row) {
  pointer-events: auto;
}

:deep(.el-pagination) {
  color: #ffffff;
}

:deep(.el-pagination button) {
  background-color: rgba(255, 255, 255, 0.1);
  color: #ffffff;
}

:deep(.el-pagination .number) {
  background-color: rgba(255, 255, 255, 0.1);
  color: #ffffff;
}

:deep(.el-pagination .number:hover) {
  background-color: rgba(255, 255, 255, 0.2);
}

/* 对话框样式 */
:deep(.article-dialog) {
  background-color: rgb(42, 89, 138);
}

:deep(.article-dialog .el-dialog) {
  background-color: rgb(42, 89, 138);
  border-radius: 8px;
}

:deep(.article-dialog .el-dialog__header) {
  background-color: rgb(42, 89, 138);
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

:deep(.article-dialog .el-dialog__title) {
  color: #ffffff;
}

:deep(.article-dialog .el-dialog__body) {
  background-color: rgb(42, 89, 138);
}

:deep(.article-dialog .el-dialog__footer) {
  background-color: rgb(42, 89, 138);
  border-top: 1px solid rgba(255, 255, 255, 0.1);
}

:deep(.article-dialog .el-form-item__label) {
  color: #ffffff;
}

:deep(.article-dialog .el-input__wrapper),
:deep(.article-dialog .el-textarea__inner) {
  background-color: rgba(255, 255, 255, 0.9);
}

:deep(.article-dialog .el-button--primary) {
  background-color: #1989fa;
  border-color: #1989fa;
}

:deep(.article-dialog .el-button--primary:hover) {
  background-color: #66b1ff;
  border-color: #66b1ff;
}

:deep(.article-dialog .el-button--default) {
  background-color: transparent;
  border-color: rgba(255, 255, 255, 0.3);
  color: #ffffff;
}

:deep(.article-dialog .el-button--default:hover) {
  background-color: rgba(255, 255, 255, 0.1);
  border-color: rgba(255, 255, 255, 0.5);
  color: #ffffff;
}

.article-form {
  padding: 10px 0;
}

.file-list {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.file-link {
  color: #66b1ff;
  text-decoration: underline;
  word-break: break-all;
}

.file-link:hover {
  color: #1989fa;
}

.file-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 5px 0;
  color: #ffffff;
}
</style>
