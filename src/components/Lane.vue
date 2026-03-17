<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue'
import Sortable from 'sortablejs'
import Card from './Card.vue'
import type { Card as CardType } from '@/types'
import { useBoard } from '@/composables/useBoard'
import Button from 'primevue/button'

const props = defineProps<{
  name: string
  cards: CardType[]
  collapsed: boolean
}>()

const { toggleLaneCollapse, moveCard } = useBoard()
const cardContainer = ref<HTMLElement | null>(null)
let sortable: Sortable | null = null

const initSortable = () => {
  if (cardContainer.value && !props.collapsed) {
    sortable = new Sortable(cardContainer.value, {
      group: 'cards',
      animation: 150,
      ghostClass: 'ghost-card',
      dragClass: 'dragging-card',
      dataIdAttr: 'data-card-id',
      onEnd: (evt) => {
        if (evt.to && evt.item) {
          const id = evt.item.getAttribute('data-card-id')!
          const newStatus = evt.to.getAttribute('data-lane')!
          const newIndex = evt.newIndex!
          
          const laneCards = Array.from(evt.to.children)
          let newPos = 1.0
          
          if (laneCards.length > 1) {
            if (newIndex === 0) {
              newPos = 0.5 
            } else if (newIndex === laneCards.length - 1) {
              newPos = laneCards.length + 1.0
            } else {
              newPos = newIndex + 0.5
            }
          }
          
          moveCard(id, newStatus, newPos)
        }
      }
    })
  }
}

onMounted(() => initSortable())

watch(() => props.collapsed, (isCollapsed) => {
  if (isCollapsed) {
    sortable?.destroy()
    sortable = null
  } else {
    setTimeout(initSortable, 0)
  }
})

onUnmounted(() => sortable?.destroy())

const formatName = (name: string) => name.replace(/-/g, ' ').toUpperCase()
</script>

<template>
  <div :class="['lane', collapsed ? 'collapsed' : 'expanded']">
    <div v-if="collapsed" class="lane-collapsed" @click="toggleLaneCollapse(name)">
      <span class="lane-name-vertical">{{ formatName(name) }}</span>
      <span class="lane-count">{{ cards.length }}</span>
    </div>

    <template v-else>
      <div class="lane-header">
        <div class="header-left">
          <h3>{{ formatName(name) }}</h3>
          <span class="lane-count">{{ cards.length }}</span>
        </div>
        <Button 
          icon="pi pi-angle-left" 
          text 
          rounded 
          size="small" 
          @click="toggleLaneCollapse(name)" 
        />
      </div>
      <div 
        ref="cardContainer" 
        class="lane-body" 
        :data-lane="name"
      >
        <Card 
          v-for="card in cards" 
          :key="card.id" 
          :card="card" 
        />
      </div>
    </template>
  </div>
</template>

<style scoped>
.lane {
  display: flex;
  flex-direction: column;
  background: var(--bg-lane);
  border-radius: var(--card-radius);
  transition: flex 0.2s ease, min-width 0.2s ease;
  height: 100%;
}

.expanded {
  flex: 1 1 0;
  min-width: 250px;
}

.collapsed {
  flex: 0 0 var(--collapsed-width);
  min-width: var(--collapsed-width);
  cursor: pointer;
  background: var(--bg-collapsed);
}

.lane-header {
  padding: 0.75rem 1rem;
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.lane-header h3 {
  margin: 0;
  font-size: 0.85rem;
  font-weight: 700;
  letter-spacing: 0.05em;
  color: var(--text-secondary);
}

.lane-count {
  font-size: 0.75rem;
  background: var(--bg-secondary);
  padding: 2px 8px;
  border-radius: 12px;
  color: var(--text-muted);
  font-weight: 600;
}

.lane-body {
  flex: 1;
  overflow-y: auto;
  padding: 0 0.75rem 0.75rem 0.75rem;
  min-height: 100px;
}

.lane-collapsed {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 1rem 0;
  height: 100%;
  gap: 1rem;
}

.lane-name-vertical {
  writing-mode: vertical-rl;
  transform: rotate(180deg);
  white-space: nowrap;
  font-weight: 700;
  font-size: 0.8rem;
  color: var(--text-muted);
}

.ghost-card {
  opacity: 0.4;
  background: var(--bg-secondary) !important;
}
</style>
