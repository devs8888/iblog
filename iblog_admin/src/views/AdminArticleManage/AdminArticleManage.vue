<script setup lang="ts">

import AdminArticleManageArea from "@/views/AdminArticleManage/components/AdminArticleManageArea.vue";
</script>

<template>
  <div class="AdminArticleManageDiv">
    <AdminArticleManageArea></AdminArticleManageArea>
  </div>
</template>

<style scoped>
.AdminArticleManageDiv {
  background-color: rgb(42, 89, 138);
  padding: 40px;
  display: flex;
  flex-direction: column;
  align-items: center;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}
</style>
