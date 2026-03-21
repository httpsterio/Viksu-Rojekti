<script setup lang="ts">
import { useBoard } from "@/composables/useBoard"
import EpicGroup from "./EpicGroup.vue"

const { config, cardsByEpic } = useBoard()
</script>

<template>
  <div v-if="config" class="epic-view">
    <EpicGroup
      v-for="epic in config.epics"
      :key="epic.name"
      :epic="epic"
      :cards="cardsByEpic[epic.name] || []"
    />

    <EpicGroup v-if="cardsByEpic['unassigned']?.length > 0" :cards="cardsByEpic['unassigned']" />
  </div>
</template>

<style scoped>
.epic-view {
  flex: 1;
  padding: 1.5rem;
  overflow-y: auto;
  background: var(--bg-secondary);
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}
</style>
