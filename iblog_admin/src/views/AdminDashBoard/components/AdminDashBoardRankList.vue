<script setup lang="ts">
import { API_BASE_URL } from '@/config'
import { ref, onMounted } from 'vue'

interface RankItem {
  Date: string
  Title: string
  PageViews: number
}

const rankList = ref<{ Rank: string; Date: string; Title: string; PageViews: string }[]>([])

const fetchRankList = async () => {
  try {
    const response = await fetch(`${API_BASE_URL}/Api/GetArticlesRankList`, {
      method: 'GET'
    })
    const data = await response.json()
    if (data.Status === 'True') {
      const list: { Rank: string; Date: string; Title: string; PageViews: string }[] = []
      if (data.Rank1) {
        list.push({ Rank: '1', ...data.Rank1 })
      }
      if (data.Rank2) {
        list.push({ Rank: '2', ...data.Rank2 })
      }
      if (data.Rank3) {
        list.push({ Rank: '3', ...data.Rank3 })
      }
      rankList.value = list
    }
  } catch (error) {
    console.error('Failed to fetch rank list:', error)
  }
}

onMounted(() => {
  fetchRankList()
})
</script>

<template>
<div class="AdminDashBoardRankListDiv">
  <el-table :data="rankList">
    <el-table-column prop="Rank" label="Rank"></el-table-column>
    <el-table-column prop="Date" label="Date"></el-table-column>
    <el-table-column prop="Title" label="Title"></el-table-column>
    <el-table-column prop="PageViews" label="PageViews"></el-table-column>
  </el-table>
</div>
</template>

<style scoped>
.AdminDashBoardRankListDiv {
  width: 100%;
  background-color: rgb(42, 89, 138);
  border-radius: 8px;
}

:deep(.el-table) {
  background-color: rgb(42, 89, 138);
  color: #ffffff;
}

:deep(.el-table__header-wrapper th) {
  background-color: rgb(42, 89, 138);
  color: #ffffff;
}

:deep(.el-table__body-wrapper tr) {
  background-color: rgb(42, 89, 138);
  color: #ffffff;
}

:deep(.el-table__row) {
  background-color: rgb(42, 89, 138);
}

:deep(.el-table__row:hover td) {
  background-color: rgba(255, 255, 255, 0.1);
}
</style>
