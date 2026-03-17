<script setup lang="ts">
import type { Card, Epic } from '@/types'
import CardComponent from './Card.vue'

defineProps<{
  epic?: Epic
  cards: Card[]
}>()
</script>

<template>
  <div class="epic-group">
    <div 
      class="epic-header" 
      :style="{ borderLeftColor: epic?.color || 'var(--text-muted)' }"
    >
      <h3>{{ epic?.name || 'Unassigned' }}</h3>
      <span class="count">{{ cards.length }}</span>
    </div>
    <div class="epic-tickets">
      <CardComponent 
        v-for="card in cards" 
        :key="card.id" 
        :card="card" 
      />
    </div>
  </div>
</template>

<style scoped>
.epic-group {
  margin-bottom: 2rem;
}

.epic-header {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 0.75rem 1rem;
  background: var(--bg-primary);
  border-left: 6px solid;
  border-radius: var(--card-radius);
  margin-bottom: 1rem;
  box-shadow: 0 1px 3px rgba(0,0,0,0.05);
}

.epic-header h3 {
  margin: 0;
  font-size: 1.1rem;
  color: var(--text-primary);
}

.count {
  font-size: 0.8rem;
  background: var(--bg-secondary);
  padding: 2px 8px;
  border-radius: 12px;
  color: var(--text-muted);
  font-weight: 600;
}

.epic-tickets {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 1rem;
}
</style>
