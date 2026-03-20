<script setup lang="ts">
import { useBoard } from "@/composables/useBoard"
import EpicGroup from "./EpicGroup.vue"

const { config, cardsByEpic } = useBoard()
</script>

<template>
  <div v-if="config" class="epic-view">
    <EpicGroup
      v-for="epic in config.epics"
      :key="epic.id"
      :epic="epic"
      :cards="cardsByEpic[epic.id] || []"
    />

    <EpicGroup v-if="cardsByEpic['unassigned']?.length > 0" :cards="cardsByEpic['unassigned']" />
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
