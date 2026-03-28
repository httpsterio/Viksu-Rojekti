<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch, nextTick } from "vue"
import { dragAndDrop } from "@formkit/drag-and-drop/vue"
import { animations, tearDown } from "@formkit/drag-and-drop"
import type {
  DragendEvent,
  DragendEventData,
  DragstartEvent,
  DragstartEventData,
} from "@formkit/drag-and-drop"
import Card from "./Card.vue"
import type { Card as CardType } from "@/types"
import { useBoard } from "@/composables/useBoard"
import Button from "primevue/button"

const props = defineProps<{
  id: string
  name: string
  cards: CardType[]
  collapsed: boolean
}>()

const { toggleStatusCollapse, moveCard, draggedCardId } = useBoard()
const cardContainer = ref<HTMLElement | undefined>(undefined)

const cardValues = ref<CardType[]>([...props.cards])
const isDragging = ref(false)

watch(
  () => props.cards,
  (cards) => {
    if (isDragging.value) return
    cardValues.value = [...cards]
  },
)

const calcPosition = (values: CardType[], index: number): number => {
  if (values.length === 1) return 1.0
  if (index === 0) return values[1].position / 2
  if (index === values.length - 1) return values[index - 1].position + 1.0
  return (values[index - 1].position + values[index + 1].position) / 2
}

const initFormKit = () => {
  if (!cardContainer.value || props.collapsed) return
  dragAndDrop({
    parent: cardContainer,
    values: cardValues,
    group: "cards",
    nativeDrag: true,
    plugins: [animations()],
    draggingClass: "dragging-card",
    dragPlaceholderClass: "ghost-card",
    onDragstart: ((data: DragstartEventData<CardType>) => {
      isDragging.value = true
      const id = data.draggedNode.data.value.id
      requestAnimationFrame(() => {
        draggedCardId.value = id
      })
    }) as DragstartEvent,
    onDragend: ((data: DragendEventData<CardType>) => {
      isDragging.value = false
      draggedCardId.value = null
      const statusId = data.parent.el.getAttribute("data-status-id")
      if (!statusId) return
      const draggedCard = data.draggedNode.data.value
      const index = data.values.findIndex((c) => c.id === draggedCard.id)
      if (index === -1) return
      moveCard(draggedCard.id, statusId, calcPosition(data.values, index))
    }) as DragendEvent,
  })
}

onMounted(() => {
  nextTick(() => initFormKit())
})

watch(
  () => props.collapsed,
  (isCollapsed) => {
    if (isCollapsed) {
      if (cardContainer.value) tearDown(cardContainer.value)
    } else {
      setTimeout(() => {
        initFormKit()
      }, 0)
    }
  },
)

onUnmounted(() => {
  if (cardContainer.value) tearDown(cardContainer.value)
})

const formatName = (name: string) => name.replace(/-/g, " ").toUpperCase()
</script>

<template>
  <div :class="['lane', collapsed ? 'collapsed' : 'expanded']">
    <div v-if="collapsed" class="lane-collapsed" @click="toggleStatusCollapse(id)">
      <span class="lane-count-vertical">{{ cards.length }}</span>
      <span class="lane-name-vertical">{{ formatName(name) }}</span>
    </div>

    <template v-else>
      <div class="lane-header">
        <Button
          icon="pi pi-angle-left"
          text
          rounded
          size="small"
          class="lane-collapse-btn"
          @click="toggleStatusCollapse(id)"
        />
        <h3>{{ formatName(name) }}</h3>
        <span class="lane-count">{{ cards.length }}</span>
      </div>
      <div ref="cardContainer" class="lane-body" :data-status-id="id">
        <Card
          v-for="card in cardValues"
          :key="card.id"
          :card="card"
          :data-pos="card.position"
          :data-card-id="card.id"
          :class="{ 'drag-placeholder': draggedCardId === card.id }"
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
  transition:
    flex 0.2s ease,
    min-width 0.2s ease;
  height: 100%;
}

.expanded {
  flex: 1 1 0;
  min-width: 16rem;
  max-width: 36rem;
}

.collapsed {
  flex: 0 0 var(--collapsed-width);
  min-width: var(--collapsed-width);
  cursor: pointer;
  background: var(--bg-collapsed);
}

.lane-header {
  padding: 0.5rem 0.75rem;
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.lane-collapse-btn {
  width: 2rem !important;
  height: 2rem !important;
}

.lane-header h3 {
  margin: 0;
  font-size: 0.85rem;
  font-weight: 700;
  letter-spacing: 0.05em;
  color: var(--text-secondary);
  flex: 1;
  text-align: center;
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
  overflow-y: scroll;
  padding: 0 0.75rem 0.75rem 0.75rem;
  min-height: 100px;
}

.lane-collapsed {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: space-between;
  padding: 1.5rem 0;
  height: 100%;
}

.lane-name-vertical {
  writing-mode: vertical-rl;
  transform: rotate(180deg);
  white-space: nowrap;
  font-weight: 700;
  font-size: 0.8rem;
  color: var(--text-muted);
}

.lane-count-vertical {
  writing-mode: vertical-rl;
  transform: rotate(180deg);
  font-size: 0.75rem;
  background: var(--bg-secondary);
  padding: 8px 4px;
  border-radius: 12px;
  color: var(--text-muted);
  font-weight: 600;
}

:deep(.ghost-card),
.drag-placeholder {
  background: transparent !important;
  border: 3px dashed #059669 !important;
  border-radius: var(--card-radius);
  box-shadow: none !important;
}

:deep(.ghost-card) *,
.drag-placeholder * {
  visibility: hidden;
}

:deep(.dragging-card) {
  cursor: grabbing !important;
  user-select: none;
  transform: rotate(20deg);
  box-shadow: 0 12px 28px rgba(0, 0, 0, 0.4);
}
</style>
