<script setup lang="ts">
import { computed } from "vue"
import { useBoard } from "@/composables/useBoard"
import EpicGroup from "./EpicGroup.vue"

const { config, cardsByEpic, activeFilters } = useBoard()

const isFiltering = computed(() =>
  !!(activeFilters.value.epic || activeFilters.value.tag || activeFilters.value.priority || activeFilters.value.search)
)

const visibleEpics = computed(() => {
  if (!config.value) return []
  if (!isFiltering.value) return config.value.epics
  return config.value.epics.filter((epic) => (cardsByEpic.value[epic.name]?.length ?? 0) > 0)
})
</script>

<template>
  <div v-if="config" class="epic-view">
    <EpicGroup
      v-for="epic in visibleEpics"
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
