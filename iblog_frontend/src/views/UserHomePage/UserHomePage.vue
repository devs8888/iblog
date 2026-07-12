<script setup lang="ts">
import UserHomePageSideBar from "@/views/UserHomePage/components/UserHomePageSideBar.vue";
import UserHomePageArticleArea from "@/views/UserHomePage/components/UserHomePageArticleArea.vue";
</script>

<template>
  <div class="UserHomePageDiv">
    <el-container>
      <el-main><UserHomePageArticleArea></UserHomePageArticleArea></el-main>
      <el-aside width="300px">
        <UserHomePageSideBar></UserHomePageSideBar>
      </el-aside>
    </el-container>
  </div>
</template>

<style scoped>
.UserHomePageDiv {
  width: 100%;
  height: 100vh;
  background-color: rgb(42, 89, 138);
}

:deep(.el-container) {
  height: 100%;
}

:deep(.el-aside) {
  height: 100%;
  background-color: rgb(42, 89, 138);
}

:deep(.el-main) {
  background-color: #ffffff;
}
</style>
