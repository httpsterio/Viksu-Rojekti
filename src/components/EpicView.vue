<script setup lang="ts">
import { useBoard } from '@/composables/useBoard'
import EpicGroup from './EpicGroup.vue'

const { config, ticketsByEpic } = useBoard()
</script>

<template>
  <div class="epic-view" v-if="config">
    <EpicGroup 
      v-for="epic in config.epics" 
      :key="epic.id"
      :epic="epic"
      :tickets="ticketsByEpic[epic.id] || []"
    />
    
    <EpicGroup 
      v-if="ticketsByEpic['unassigned']?.length > 0"
      :tickets="ticketsByEpic['unassigned']"
    />
  </div>
</template>

<style scoped>
.epic-view {
  flex: 1;
  padding: 2rem;
  overflow-y: auto;
  background: var(--bg-secondary);
}
</style>
