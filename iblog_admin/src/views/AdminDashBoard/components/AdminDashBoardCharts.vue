<script setup lang="ts">
import { API_BASE_URL } from '@/config'
import { onMounted, ref } from 'vue'
import * as echarts from 'echarts'

const pageViewsData = ref<number[]>([])
const newArticlesData = ref<number[]>([])

const getDateDaysAgo = (daysAgo: number) => {
  const now = new Date()
  now.setDate(now.getDate() - daysAgo)
  const year = now.getFullYear()
  const month = String(now.getMonth() + 1).padStart(2, '0')
  const day = String(now.getDate()).padStart(2, '0')
  return `${year}-${month}-${day}`
}

const fetchChartData = async () => {
  const days = [4, 3, 2, 1, 0]
  const promises = days.map(async (daysAgo) => {
    const date = getDateDaysAgo(daysAgo)
    const [pvRes, naRes] = await Promise.all([
      fetch(`${API_BASE_URL}/Api/GetNewArticlesPageViewsByTime`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ Time: date })
      }).then(r => r.json()),
      fetch(`${API_BASE_URL}/Api/GetNewArticlesNumberByTime`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ Time: date })
      }).then(r => r.json())
    ])
    return {
      pageViews: pvRes.Status === 'True' ? (pvRes.PageViews || 0) : 0,
      newArticles: naRes.Status === 'True' ? (naRes.Number || 0) : 0
    }
  })
  const results = await Promise.all(promises)
  pageViewsData.value = results.map(r => r.pageViews)
  newArticlesData.value = results.map(r => r.newArticles)
}

const initCharts = () => {
  // PageViews Line Chart
  const pageViewsChart = echarts.init(document.getElementById('AdminDashBoardPageViewsChart')!)
  pageViewsChart.setOption({
    title: { text: 'PageViewsChart', left: 'center', textStyle: { color: '#000000' } },
    tooltip: {},
    xAxis: {
      type: 'category',
      data: ['4DayAgo', '3DayAgo', '2DayAgo', '1DayAgo', 'Today'],
      axisLabel: { color: '#000000' }
    },
    yAxis: {
      type: 'value',
      axisLabel: { color: '#000000' }
    },
    series: [{
      data: pageViewsData.value,
      type: 'line',
      smooth: true
    }]
  })

  // NewArticles Bar Chart
  const newArticlesChart = echarts.init(document.getElementById('AdminDashBoardNewArticlesChart')!)
  newArticlesChart.setOption({
    title: { text: 'NewArticlesChart', left: 'center', textStyle: { color: '#000000' } },
    tooltip: {},
    xAxis: {
      type: 'category',
      data: ['4DayAgo', '3DayAgo', '2DayAgo', '1DayAgo', 'Today'],
      axisLabel: { color: '#000000' }
    },
    yAxis: {
      type: 'value',
      axisLabel: { color: '#000000' }
    },
    series: [{
      data: newArticlesData.value,
      type: 'bar'
    }]
  })
}

onMounted(async () => {
  await fetchChartData()
  initCharts()
})
</script>

<template>
  <div class="ChartsContainer">
    <div id="AdminDashBoardPageViewsChart" class="AdminDashBoardPageViewsChart"></div>
    <div id="AdminDashBoardNewArticlesChart" class="AdminDashBoardNewArticlesChart"></div>
  </div>
</template>

<style scoped>
.ChartsContainer {
  display: flex;
  gap: 20px;
  width: 100%;
}

.AdminDashBoardPageViewsChart,
.AdminDashBoardNewArticlesChart {
  width: 50%;
  height: 300px;
}
</style>
