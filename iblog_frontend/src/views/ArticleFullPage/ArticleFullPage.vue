<script setup lang="ts">
import ArticleFullPageShowArea from '@/views/ArticleFullPage/components/ArticleFullPageShowArea.vue'
import { useRouter } from 'vue-router'

const router = useRouter()

const goBack = () => {
  router.push({ name: 'UserHomePage' })
}
</script>

<template>
  <div class="ArticleFullPageDiv">
    <el-container>
      <el-header class="HeaderDiv">
        <el-button @click="goBack" class="BackButton">Back</el-button>
      </el-header>
      <el-main>
        <ArticleFullPageShowArea></ArticleFullPageShowArea>
      </el-main>
    </el-container>
  </div>
</template>

<style scoped>
.ArticleFullPageDiv {
  width: 100%;
  height: 100vh;
  background-color: rgb(42, 89, 138);
}

:deep(.el-container) {
  height: 100%;
}

:deep(.el-header) {
  background-color: rgb(42, 89, 138);
  display: flex;
  align-items: center;
}

:deep(.el-main) {
  background-color: #ffffff;
}

.BackButton {
  background-color: rgb(42, 89, 138);
  color: #ffffff;
  border: none;
}

.BackButton:hover {
  background-color: rgb(60, 120, 180);
  color: #ffffff;
}
</style>
