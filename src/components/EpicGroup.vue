<script setup lang="ts">
import { ref, computed } from "vue"
import type { Card, Epic } from "@/types"
import CardComponent from "./Card.vue"
import { useBoard } from "@/composables/useBoard"

const props = defineProps<{
  epic?: Epic
  cards: Card[]
}>()

const { doneStatusNames } = useBoard()

const activeCards = computed(() => props.cards.filter((c) => !doneStatusNames.value.has(c.status)))
const doneCards = computed(() => props.cards.filter((c) => doneStatusNames.value.has(c.status)))

const doneExpanded = ref(false)
</script>

<template>
  <div class="epic-group" :style="{ borderLeftColor: epic?.color || 'var(--text-muted)' }">
    <div class="epic-header">
      <h3>{{ epic?.name || "Unassigned" }}</h3>
      <span class="count">{{ activeCards.length }}</span>
    </div>
    <div class="epic-cards">
      <CardComponent v-for="card in activeCards" :key="card.id" :card="card" />
    </div>
    <button v-if="doneCards.length > 0" class="done-toggle" @click="doneExpanded = !doneExpanded">
      <i :class="doneExpanded ? 'pi pi-chevron-down' : 'pi pi-chevron-right'" />
      {{ doneExpanded ? "Hide" : "Show" }} {{ doneCards.length }} done {{ doneCards.length === 1 ? "item" : "items" }}
    </button>
    <div v-if="doneExpanded" class="epic-cards done-cards">
      <CardComponent v-for="card in doneCards" :key="card.id" :card="card" />
    </div>
  </div>
</template>

<style scoped>
.epic-group {
  background: var(--bg-lane);
  border-left: 6px solid;
  border-radius: var(--card-radius);
  padding: 1.25rem;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.epic-header {
  display: flex;
  align-items: center;
  gap: 1rem;
  margin-bottom: 1.25rem;
}

.epic-header h3 {
  margin: 0;
  font-size: 1.1rem;
  color: var(--text-primary);
  font-weight: 700;
}

.count {
  font-size: 0.8rem;
  background: var(--bg-primary);
  padding: 2px 10px;
  border-radius: 12px;
  color: var(--text-secondary);
  font-weight: 600;
  border: 1px solid var(--border-color);
}

.epic-cards {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 1rem;
}

.done-toggle {
  display: flex;
  align-items: center;
  gap: 0.4rem;
  margin-top: 1rem;
  background: none;
  border: none;
  cursor: pointer;
  font-size: 0.8rem;
  color: var(--text-muted);
  padding: 0;
}

.done-toggle:hover {
  color: var(--text-secondary);
}

.done-cards {
  margin-top: 0.75rem;
}

@media (max-width: 600px) {
  .epic-cards {
    grid-template-columns: 1fr;
  }
}
</style>
