<script setup lang="ts">

import AdminSecurityMCPSetting from "@/views/AdminSecuritySetting/components/AdminSecurityMCPSetting.vue";
import AdminSecurityAccountSetting from "@/views/AdminSecuritySetting/components/AdminSecurityAccountSetting.vue";
import AdminSecurityLogView from "@/views/AdminSecuritySetting/components/AdminSecurityLogView.vue";
</script>

<template>
<div class="AdminSecuritySettingDiv">
  <AdminSecurityMCPSetting></AdminSecurityMCPSetting>
  <AdminSecurityAccountSetting></AdminSecurityAccountSetting>
  <AdminSecurityLogView></AdminSecurityLogView>
</div>
</template>

<style scoped>
.AdminSecuritySettingDiv {
  background-color: rgb(42, 89, 138);
  padding: 40px;
  display: flex;
  flex-direction: column;
  align-items: center;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}
</style>
