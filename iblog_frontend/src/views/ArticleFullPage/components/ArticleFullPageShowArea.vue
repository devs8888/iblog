<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import { API_BASE_URL } from '@/config'

interface ArticleVersionItem {
  Version: number
  Date: string
  Title: string
  Content: string
  PageViews: number
}

interface ArticleFullContent {
  Id: string
  Sender: string
  Version: number
  Date: string
  Title: string
  Content: string
  PageViews: number
  File: string | null
}

interface FileListResponse {
  Status: string
  [key: string]: string | undefined
}

const route = useRoute()
const articleId = ref(route.params.id as string)

const versionList = ref<ArticleVersionItem[]>([])
const selectedVersion = ref<number | null>(null)
const articleFullContent = ref<ArticleFullContent | null>(null)
const loading = ref(false)
const contentLoading = ref(false)

const fetchVersionList = async () => {
  loading.value = true
  try {
    const response = await fetch(`${API_BASE_URL}/Api/GetArticleVersion`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ Id: articleId.value })
    })
    const data = await response.json()
    if (data.Status === 'True' && data.ArticleInfo) {
      versionList.value = data.ArticleInfo
      if (versionList.value.length > 0) {
        selectVersion(versionList.value[0].Version)
      }
    }
  } catch {
    versionList.value = []
  } finally {
    loading.value = false
  }
}

const fileList = ref<string[]>([])

const selectVersion = async (version: number) => {
  selectedVersion.value = version
  contentLoading.value = true
  fileList.value = []
  try {
    const articleResponse = await fetch(`${API_BASE_URL}/Api/GetArticleFull`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ Id: articleId.value, Version: version })
    })
    const articleData = await articleResponse.json()
    if (articleData.Status === 'True' && articleData.Article) {
      articleFullContent.value = articleData.Article

      const fileResponse = await fetch(`${API_BASE_URL}/Api/GetArticleFile?Id=${articleId.value}&Version=${version}`)
      const fileData: FileListResponse = await fileResponse.json()
      if (fileData.Status === 'True') {
        const files: string[] = []
        for (let i = 1; ; i++) {
          const fileKey = `File${i}`
          if (fileData[fileKey]) {
            files.push(fileData[fileKey] as string)
          } else {
            break
          }
        }
        fileList.value = files
      }
    } else {
      articleFullContent.value = null
    }
  } catch {
    articleFullContent.value = null
  } finally {
    contentLoading.value = false
  }
}

const downloadFileByName = (fileName: string) => {
  const fullUrl = `${API_BASE_URL}/Api/GetArticleFile?Id=${articleId.value}&Version=${selectedVersion.value}&FileName=${encodeURIComponent(fileName)}`
  window.open(fullUrl, '_blank')
}

onMounted(async () => {
  await fetchVersionList()
})
</script>

<template>
  <div class="ShowAreaDiv">
    <el-card class="VersionSelectCard">
      <template #header>
        <div class="VersionSelectHeader">Version List</div>
      </template>
      <div v-if="loading" class="LoadingDiv">Loading...</div>
      <div v-else class="VersionListDiv">
        <div
          v-for="version in versionList"
          :key="version.Version"
          class="VersionItem"
          :class="{ 'VersionItemActive': selectedVersion === version.Version }"
          @click="selectVersion(version.Version)"
        >
          <span>v{{ version.Version }}</span>
          <span class="VersionTitle">{{ version.Title }}</span>
        </div>
      </div>
    </el-card>

    <el-card v-if="selectedVersion !== null" class="ArticleContentCard">
      <template #header>
        <div class="ArticleTitle">{{ articleFullContent?.Title }}</div>
      </template>
      <div v-if="contentLoading" class="LoadingDiv">Loading...</div>
      <div v-else class="ArticleContentDiv">
        <el-divider class="CustomDivider"></el-divider>
        <div class="ContentText">{{ articleFullContent?.Content }}</div>
        <el-divider class="CustomDivider"></el-divider>
        <div class="FileSection">
          <div class="FileSectionTitle">File List:</div>
          <div v-if="fileList.length > 0" class="FileListDiv">
            <div
              v-for="(fileName, index) in fileList"
              :key="index"
              class="FileItem"
              @click="downloadFileByName(fileName)"
            >
              {{ fileName }}
            </div>
          </div>
          <div v-else class="FileNullText">null</div>
        </div>
      </div>
    </el-card>
  </div>
</template>

<style scoped>
.ShowAreaDiv {
  width: 100%;
  display: flex;
  flex-direction: column;
  gap: 20px;
  padding: 20px;
}

.VersionSelectCard {
  background-color: rgb(42, 89, 138);
  color: #ffffff;
}

:deep(.VersionSelectCard .el-card__header) {
  background-color: rgb(42, 89, 138);
  color: #ffffff;
}

:deep(.VersionSelectCard .el-card__body) {
  background-color: rgb(42, 89, 138);
  color: #ffffff;
}

.VersionSelectHeader {
  font-size: 18px;
  font-weight: bold;
}

.LoadingDiv {
  color: #ffffff;
  text-align: center;
  padding: 20px;
}

.VersionListDiv {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.VersionItem {
  padding: 10px 15px;
  background-color: rgba(255, 255, 255, 0.2);
  border-radius: 4px;
  cursor: pointer;
  display: flex;
  gap: 10px;
  align-items: center;
}

.VersionItem:hover {
  background-color: rgba(255, 255, 255, 0.4);
}

.VersionItemActive {
  background-color: rgba(255, 255, 255, 0.4);
  border: 2px solid #ffffff;
}

.VersionTitle {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.ArticleContentCard {
  background-color: rgb(42, 89, 138);
  color: #ffffff;
}

:deep(.ArticleContentCard .el-card__header) {
  background-color: rgb(42, 89, 138);
  color: #ffffff;
  text-align: center;
}

:deep(.ArticleContentCard .el-card__body) {
  background-color: rgb(42, 89, 138);
  color: #ffffff;
}

.ArticleTitle {
  font-size: 24px;
  font-weight: bold;
  text-align: center;
}

.CustomDivider {
  border-color: rgba(255, 255, 255, 0.5);
}

.ContentText {
  white-space: pre-wrap;
  line-height: 1.8;
  padding: 20px 0;
}

.FileSection {
  padding: 10px 0;
}

.FileSectionTitle {
  font-weight: bold;
  margin-bottom: 10px;
}

.FileListDiv {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.FileItem {
  padding: 8px 15px;
  background-color: rgba(255, 255, 255, 0.2);
  border-radius: 4px;
  cursor: pointer;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.FileItem:hover {
  background-color: rgba(255, 255, 255, 0.4);
}

.FileNullText {
  color: rgba(255, 255, 255, 0.6);
  font-style: italic;
}
</style>
