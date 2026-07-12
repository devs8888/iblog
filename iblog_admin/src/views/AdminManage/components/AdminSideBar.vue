<script setup lang="ts">

</script>

<template>
  <el-menu default-active="2" class="AdminSideBar">
    <el-menu-item index="1">
      <span @click="$emit('menu-select', 'dashboard')">DashBoard</span>
    </el-menu-item>
    <el-sub-menu index="2">
      <template #title>
        <span>Account Setting</span>
      </template>
      <el-menu-item index="2-1" @click="$emit('menu-select', 'profile')">Profile Setting</el-menu-item>
      <el-menu-item index="2-2" @click="$emit('menu-select', 'security')">Security Setting</el-menu-item>
    </el-sub-menu>
    <el-sub-menu index="3">
      <template #title>
        <span>Content Setting</span>
      </template>
      <el-menu-item index="3-1" @click="$emit('menu-select', 'articlemanage')">Article Manage</el-menu-item>
    </el-sub-menu>
    <el-menu-item index="4" @click="$emit('menu-select', 'sitesetting')">Website Setting</el-menu-item>
  </el-menu>
</template>

<style scoped>
.AdminSideBar {
  background-color: rgb(42, 89, 138);
  color: #ffffff;
  height: 100%;
  border: none;
}

.AdminSideBar :deep(.el-menu-item),
.AdminSideBar :deep(.el-sub-menu__title) {
  color: #ffffff;
  background-color: rgb(42, 89, 138);
}

.AdminSideBar :deep(.el-menu-item:hover),
.AdminSideBar :deep(.el-sub-menu__title:hover) {
  background-color: rgba(255, 255, 255, 0.1);
}

.AdminSideBar :deep(.el-sub-menu .el-menu) {
  background-color: rgb(42, 89, 138);
}

.AdminSideBar :deep(.el-sub-menu .el-menu-item) {
  background-color: rgb(42, 89, 138);
}
</style>
