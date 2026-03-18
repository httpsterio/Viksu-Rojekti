<script setup lang="ts">
import { useBoard } from '@/composables/useBoard'
import Lane from './Lane.vue'

const { config, cardsByStatus, collapsedStatuses } = useBoard()
</script>

<template>
  <div class="board-view" v-if="config">
    <Lane 
      v-for="status in config.statuses" 
      :key="status.id"
      :id="status.id"
      :name="status.name"
      :cards="cardsByStatus[status.id] || []"
      :collapsed="collapsedStatuses.has(status.id)"
    />
  </div>
</template>

<style scoped>
.board-view {
  flex: 1;
  display: flex;
  gap: var(--lane-gap);
  padding: var(--lane-gap);
  overflow-x: auto;
  background: var(--bg-secondary);
}
</style>
