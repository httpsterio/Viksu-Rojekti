<script setup lang="ts">
import { useBoard } from '@/composables/useBoard'
import Lane from './Lane.vue'

const { config, ticketsByLane, collapsedLanes } = useBoard()
</script>

<template>
  <div class="board-view" v-if="config">
    <Lane 
      v-for="lane in config.lanes" 
      :key="lane"
      :name="lane"
      :tickets="ticketsByLane[lane] || []"
      :collapsed="collapsedLanes.has(lane)"
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
