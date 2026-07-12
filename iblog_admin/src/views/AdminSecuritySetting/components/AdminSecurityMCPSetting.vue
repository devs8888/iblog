<script setup lang="ts">
import { API_BASE_URL } from '@/config'
import { ref, onMounted } from 'vue'
import { ElMessage } from 'element-plus'

interface McpKeyItem {
  Date: string
  Key: string
  Auth_Read: number
  Auth_Add: number
  Auth_Change: number
  Auth_Remove: number
}

const mcpKeyList = ref<McpKeyItem[]>([])
const currentPage = ref(1)
const totalMcpKeys = ref(0)
const pageSize = 3
const dialogVisible = ref(false)
const generatedKey = ref('')
const showKeyDialog = ref(false)
const loading = ref(false)

const checkedAuth = ref({
  Auth_Read: 1,
  Auth_Add: 0,
  Auth_Change: 0,
  Auth_Remove: 0
})

const loadMcpKeys = async () => {
  loading.value = true
  try {
    const response = await fetch(`${API_BASE_URL}/Api/AdminGetMcpKeys`, {
      method: 'POST',
      credentials: 'include',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({ Page: currentPage.value })
    })
    const data = await response.json()
    if (data.Status === 'True' && data.McpKeys) {
      mcpKeyList.value = data.McpKeys
    } else {
      mcpKeyList.value = []
    }
  } catch {
    mcpKeyList.value = []
  } finally {
    loading.value = false
  }
}

const fetchMcpKeyNumber = async () => {
  try {
    const response = await fetch(`${API_BASE_URL}/Api/AdminGetMcpKeyNumber`, {
      method: 'GET',
      credentials: 'include'
    })
    const data = await response.json()
    if (data.Status === 'True') {
      totalMcpKeys.value = data.Number || 0
    }
  } catch {
    totalMcpKeys.value = 0
  }
}

const handlePageChange = (page: number) => {
  currentPage.value = page
  loadMcpKeys()
}

const handleAddClick = () => {
  checkedAuth.value = {
    Auth_Read: 1,
    Auth_Add: 0,
    Auth_Change: 0,
    Auth_Remove: 0
  }
  dialogVisible.value = true
}

const handleSubmit = async () => {
  try {
    const response = await fetch(`${API_BASE_URL}/Api/AdminAddMcpKey`, {
      method: 'POST',
      credentials: 'include',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        Auth_Read: checkedAuth.value.Auth_Read ? 1 : 0,
        Auth_Add: checkedAuth.value.Auth_Add ? 1 : 0,
        Auth_Change: checkedAuth.value.Auth_Change ? 1 : 0,
        Auth_Remove: checkedAuth.value.Auth_Remove ? 1 : 0
      })
    })
    const data = await response.json()
    if (data.Status === 'True') {
      generatedKey.value = data.Key
      showKeyDialog.value = true
      dialogVisible.value = false
      await fetchMcpKeyNumber()
      await loadMcpKeys()
    } else {
      ElMessage.error('Add MCP Key failed')
    }
  } catch {
    ElMessage.error('Add MCP Key failed')
  }
}

const handleCopyKey = () => {
  navigator.clipboard.writeText(generatedKey.value).then(() => {
    ElMessage.success('Key copied to clipboard')
  }).catch(() => {
    ElMessage.error('Copy failed')
  })
}

const handleDelete = async (key: string) => {
  try {
    const response = await fetch(`${API_BASE_URL}/Api/AdminDeleteMcpKey`, {
      method: 'POST',
      credentials: 'include',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({ Key: key })
    })
    const data = await response.json()
    if (data.Status === 'True') {
      ElMessage.success('Delete MCP Key success')
      await fetchMcpKeyNumber()
      await loadMcpKeys()
    } else {
      ElMessage.error('Delete MCP Key failed')
    }
  } catch {
    ElMessage.error('Delete MCP Key failed')
  }
}

const handleAuthChange = async (key: string, authField: string, value: boolean) => {
  const row = mcpKeyList.value.find(k => k.Key === key)
  if (!row) return

  // Update local state immediately
  if (authField === 'Auth_Add') row.Auth_Add = value ? 1 : 0
  if (authField === 'Auth_Change') row.Auth_Change = value ? 1 : 0
  if (authField === 'Auth_Remove') row.Auth_Remove = value ? 1 : 0

  try {
    const response = await fetch(`${API_BASE_URL}/Api/AdminChangeMcpKey`, {
      method: 'POST',
      credentials: 'include',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        Key: key,
        Auth_Read: row.Auth_Read,
        Auth_Add: row.Auth_Add,
        Auth_Change: row.Auth_Change,
        Auth_Remove: row.Auth_Remove
      })
    })
    const data = await response.json()
    if (data.Status !== 'True') {
      ElMessage.error('Change MCP Key auth failed')
      await loadMcpKeys()
    }
  } catch {
    ElMessage.error('Change MCP Key auth failed')
    await loadMcpKeys()
  }
}

onMounted(async () => {
  await fetchMcpKeyNumber()
  await loadMcpKeys()
})
</script>

<template>
  <div class="AdminSecurityMCPSettingDiv">
    <el-table :data="mcpKeyList" v-loading="loading">
      <el-table-column prop="Date" label="Date"></el-table-column>
      <el-table-column prop="Key" label="Key"></el-table-column>
      <el-table-column label="Auth">
        <template #default="scope">
          <el-checkbox disabled :model-value="scope.row.Auth_Read === 1">Read</el-checkbox>
          <el-checkbox :model-value="scope.row.Auth_Add === 1" @change="(val: boolean) => handleAuthChange(scope.row.Key, 'Auth_Add', val)">Add</el-checkbox>
          <el-checkbox :model-value="scope.row.Auth_Change === 1" @change="(val: boolean) => handleAuthChange(scope.row.Key, 'Auth_Change', val)">Change</el-checkbox>
          <el-checkbox :model-value="scope.row.Auth_Remove === 1" @change="(val: boolean) => handleAuthChange(scope.row.Key, 'Auth_Remove', val)">Remove</el-checkbox>
        </template>
      </el-table-column>
      <el-table-column label="Delete">
        <template #default="scope">
          <el-button type="primary" plain @click="handleDelete(scope.row.Key)">Delete</el-button>
        </template>
      </el-table-column>
    </el-table>

    <el-pagination
      v-if="totalMcpKeys > 0"
      :current-page="currentPage"
      :page-size="pageSize"
      :total="totalMcpKeys"
      layout="prev, pager, next"
      @current-change="handlePageChange"
      class="pagination"
      style="justify-content: center"
    ></el-pagination>

    <el-button type="primary" @click="handleAddClick">Add MCP Key</el-button>

    <!-- Add MCP Key Dialog -->
    <el-dialog v-model="dialogVisible" title="Add MCP Key" width="500px">
      <el-checkbox v-model="checkedAuth.Auth_Read" :disabled="true">Read (forced)</el-checkbox>
      <el-checkbox v-model="checkedAuth.Auth_Add">Add</el-checkbox>
      <el-checkbox v-model="checkedAuth.Auth_Change">Change</el-checkbox>
      <el-checkbox v-model="checkedAuth.Auth_Remove">Remove</el-checkbox>
      <template #footer>
        <el-button @click="dialogVisible = false">Cancel</el-button>
        <el-button type="primary" @click="handleSubmit">Submit</el-button>
      </template>
    </el-dialog>

    <!-- Show Generated Key Dialog -->
    <el-dialog v-model="showKeyDialog" title="MCP Key Generated" width="500px">
      <p style="color: #ff6b6b;">Warning: This key will only be displayed once. Please copy and save it safely.</p>
      <p style="word-break: break-all; font-family: monospace; background: #f5f5f5; padding: 10px; border-radius: 4px;">{{ generatedKey }}</p>
      <template #footer>
        <el-button @click="showKeyDialog = false">Close</el-button>
        <el-button type="primary" @click="handleCopyKey">Copy Key</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<style scoped>
.AdminSecurityMCPSettingDiv {
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

:deep(.el-table__row) {
  pointer-events: none;
}

:deep(.el-table__row td) {
  pointer-events: none;
}

:deep(.el-table__row .el-checkbox) {
  pointer-events: auto;
}

:deep(.el-table__row .el-checkbox.is-disabled) {
  pointer-events: none;
}

:deep(.el-table__row .el-button) {
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

:deep(.el-dialog) {
  background-color: rgb(42, 89, 138);
}

:deep(.el-dialog__title) {
  color: #ffffff;
}

:deep(.el-dialog__body) {
  color: #ffffff;
}

:deep(.el-checkbox__label) {
  color: #000000;
}
</style>
