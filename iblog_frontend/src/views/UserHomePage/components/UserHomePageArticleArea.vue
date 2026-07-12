<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { API_BASE_URL } from '@/config'

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

const router = useRouter()
const articleData = ref<Article[]>([])
const currentPage = ref(1)
const totalArticles = ref(0)
const pageSize = 3
const loading = ref(false)
const searchKeyword = ref('')

const goToArticleFullPage = (id: string) => {
  router.push({ name: 'ArticleFullPage', params: { id } })
}

const fetchArticleNumber = async (keyword?: string) => {
  try {
    const response = await fetch(`${API_BASE_URL}/Api/GetArticleNumber`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ KeyWord: keyword || null })
    })
    const data = await response.json()
    if (data.Status === 'True') {
      totalArticles.value = data.Number || 0
    }
  } catch {
    totalArticles.value = 0
  }
}

const fetchArticleInfo = async (page: number, keyword?: string) => {
  loading.value = true
  try {
    const response = await fetch(`${API_BASE_URL}/Api/GetArticleInfo`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ Page: page, KeyWord: keyword || null })
    })
    const data = await response.json()
    if (data.Status === 'True' && data.ArticleInfo) {
      articleData.value = data.ArticleInfo
    } else {
      articleData.value = []
    }
  } catch {
    articleData.value = []
  } finally {
    loading.value = false
  }
}

const handleSearch = async () => {
  currentPage.value = 1
  const keyword = searchKeyword.value.trim() || undefined
  await fetchArticleNumber(keyword)
  await fetchArticleInfo(1, keyword)
}

const handlePageChange = (page: number) => {
  currentPage.value = page
  const keyword = searchKeyword.value.trim() || undefined
  fetchArticleInfo(page, keyword)
}

onMounted(async () => {
  await fetchArticleNumber()
  await fetchArticleInfo(1)
})
</script>

<template>
  <div class="ArticleAreaDiv">
    <div class="SearchBar">
      <el-input v-model="searchKeyword" placeholder="Search articles..." class="SearchInput"></el-input>
      <el-button type="primary" @click="handleSearch" class="SearchButton">Search</el-button>
    </div>

    <el-card v-for="article in articleData" :key="article.Id" class="ArticleCard" @click="goToArticleFullPage(article.Id)">
      <template #header>
        <div class="ArticleTitleHeader">
          <span>{{ article.Title }}</span><span> - </span><span>{{ article.Id }}</span>
        </div>
      </template>
      <p>Content: {{ article.Content }}</p><br>
      <p>Sender: {{ article.Sender }}</p><br>
      <p>Version: {{ article.Version }}</p>
      <template #footer>
        <p>Date: {{ article.Date }}</p><br>
        <p>PageViews: {{ article.PageViews }}</p>
      </template>
    </el-card>

    <el-pagination
      v-if="totalArticles > 0"
      :current-page="currentPage"
      :page-size="pageSize"
      :total="totalArticles"
      layout="prev, pager, next"
      @current-change="handlePageChange"
      class="pagination"
      style="justify-content: center"
    ></el-pagination>
  </div>
</template>

<style scoped>
.ArticleAreaDiv {
  width: 100%;
  display: flex;
  flex-direction: column;
  gap: 20px;
  padding: 20px;
}

.SearchBar {
  display: flex;
  gap: 10px;
  justify-content: center;
}

.SearchInput {
  max-width: 300px;
}

.SearchButton {
  background-color: rgb(42, 89, 138);
  border-color: rgb(42, 89, 138);
}

.ArticleCard {
  background-color: rgb(42, 89, 138);
  color: #ffffff;
}

:deep(.el-card__header) {
  background-color: rgb(42, 89, 138);
  color: #ffffff;
}

:deep(.el-card__body) {
  color: #ffffff;
}

:deep(.el-card__footer) {
  color: #ffffff;
}

:deep(.el-pagination) {
  color: #ffffff;
}

:deep(.el-pagination button) {
  background-color: rgb(42, 89, 138);
  color: #ffffff;
}

:deep(.el-pagination .number) {
  background-color: rgb(42, 89, 138);
  color: #ffffff;
}

:deep(.el-pagination .number:hover) {
  background-color: rgb(60, 120, 180);
}

:deep(.el-pagination .is-active) {
  background-color: rgb(30, 70, 110);
  color: #ffffff;
}

:deep(.el-pagination .el-icon) {
  color: #ffffff;
}

:deep(.el-pagination .more) {
  color: #ffffff;
}
</style>
