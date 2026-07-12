<script setup lang="ts">
import { API_BASE_URL } from '@/config'
import { ref, onMounted } from 'vue'
import { ElMessage } from 'element-plus'

interface LogItem {
  Date: string
  Operator: string
  Action: string
  Object: string
}

const logData = ref<LogItem[]>([])
const currentPage = ref(1)
const totalLogs = ref(0)
const pageSize = 3
const loading = ref(false)

const fetchLogNumber = async () => {
  try {
    const response = await fetch(`${API_BASE_URL}/Api/GetLogNumber`, {
      method: 'GET',
      credentials: 'include'
    })
    const data = await response.json()
    if (data.Status === 'True') {
      totalLogs.value = data.Number || 0
    }
  } catch {
    totalLogs.value = 0
  }
}

const fetchLogInfo = async (page: number) => {
  loading.value = true
  try {
    const response = await fetch(`${API_BASE_URL}/Api/GetLogInfo`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      credentials: 'include',
      body: JSON.stringify({ Page: page })
    })
    const data = await response.json()
    if (data.Status === 'True' && data.LogInfo) {
      logData.value = data.LogInfo
    } else {
      logData.value = []
    }
  } catch {
    logData.value = []
  } finally {
    loading.value = false
  }
}

const handlePageChange = (page: number) => {
  currentPage.value = page
  fetchLogInfo(page)
}

const handleDelete = async () => {
  try {
    const response = await fetch(`${API_BASE_URL}/Api/AdminDeleteLog`, {
      method: 'POST',
      credentials: 'include'
    })
    const data = await response.json()
    if (data.Status === 'True') {
      ElMessage.success('Logs deleted successfully')
      await fetchLogNumber()
      await fetchLogInfo(1)
      currentPage.value = 1
    } else {
      ElMessage.error('Failed to delete logs')
    }
  } catch {
    ElMessage.error('Failed to delete logs')
  }
}

onMounted(async () => {
  await fetchLogNumber()
  await fetchLogInfo(1)
})
</script>

<template>
  <div class="AdminSecurityLogViewDiv">
    <el-table :data="logData" v-loading="loading">
      <el-table-column prop="Date" label="Date"></el-table-column>
      <el-table-column prop="Operator" label="Operator"></el-table-column>
      <el-table-column prop="Action" label="Action"></el-table-column>
      <el-table-column prop="Object" label="Object"></el-table-column>
    </el-table>

    <el-pagination
      v-if="totalLogs > 0"
      :current-page="currentPage"
      :page-size="pageSize"
      :total="totalLogs"
      layout="prev, pager, next"
      @current-change="handlePageChange"
      class="pagination"
      style="justify-content: center"
    ></el-pagination>

    <el-button type="primary" @click="handleDelete" style="margin-top: 20px">Delete</el-button>
  </div>
</template>

<style scoped>
.AdminSecurityLogViewDiv {
  color: #ffffff;
  width: 100%;
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
  pointer-events: none;
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
</style>
