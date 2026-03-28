<script setup lang="ts">
import { computed } from "vue"
import { useBoard } from "@/composables/useBoard"
import { useColorContrast } from "@/composables/useColorContrast"
import type { Card } from "@/types"

const props = defineProps<{
  card: Card
}>()

const { config, doneStatusNames, editingCard } = useBoard()
const { contrastColor } = useColorContrast()

const isDone = computed(() => doneStatusNames.value.has(props.card.status))

const epic = computed(() => config.value?.epics.find((e) => e.name === props.card.epic))

const dueDateBorderColor = computed(() => {
  if (!props.card.dueDate || isDone.value) return "transparent"
  const today = new Date()
  today.setHours(0, 0, 0, 0)
  const [dy, dm, dd] = props.card.dueDate.split('-').map(Number)
  const due = new Date(dy, dm - 1, dd)
  const diffDays = Math.ceil((due.getTime() - today.getTime()) / (1000 * 60 * 60 * 24))
  const threshold = config.value?.dueDateThreshold ?? 7
  if (diffDays < 0) return "#E53E3E" // overdue — red
  if (diffDays <= threshold) return "#ECC94B" // due soon — yellow
  return "transparent"
})

const checklistInfo = computed(() => {
  if (!props.card.checklist || props.card.checklist.length === 0) return null
  const total = props.card.checklist.length
  const done = props.card.checklist.filter((i) => i.done).length
  return {
    text: `${done}/${total}`,
    isComplete: done === total,
  }
})

const getTag = (name: string) => config.value?.tags.find((t) => t.name === name)

const getTagStyle = (name: string) => {
  const tag = getTag(name)
  if (tag?.color) {
    return {
      backgroundColor: tag.color,
      color: contrastColor(tag.color),
      borderColor: "transparent",
    }
  }
  return {}
}
</script>

<template>
  <div
    class="card"
    :class="{ 'card-done': isDone, 'priority-0': card.priority === 0 }"
    :style="[
      card.priority > 0 && config
        ? { borderLeftColor: config.priorities[card.priority - 1]?.color }
        : {},
      { borderRightColor: dueDateBorderColor },
    ]"
    :data-card-id="card.id"
    @click="editingCard = card"
  >
    <div class="card-header">
      <span class="card-id">{{ card.id }}</span>
      <div
        v-if="checklistInfo"
        class="checklist-progress"
        :class="{ 'checklist-complete': checklistInfo.isComplete }"
      >
        <i class="pi pi-check-square"></i>
        <span>{{ checklistInfo.text }}</span>
      </div>
    </div>
    <h4 class="card-title">{{ card.title }}</h4>
    <div class="card-meta">
      <span
        v-if="epic"
        class="epic-badge"
        :style="{ backgroundColor: epic.color, color: contrastColor(epic.color) }"
      >
        {{ epic.name }}
      </span>
      <span
        v-for="tagName in card.tags"
        :key="tagName"
        class="tag-pill"
        :style="getTagStyle(tagName)"
      >
        {{ getTag(tagName)?.name || tagName }}
      </span>
    </div>
  </div>
</template>

<style scoped>
.card {
  background: var(--bg-card);
  border-left: 4px solid transparent;
  border-right: 4px solid transparent;
  border-radius: var(--card-radius);
  padding: 0.75rem;
  margin-bottom: 0.75rem;
  cursor: pointer;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  transition:
    transform 0.1s,
    box-shadow 0.1s;
}

.card:hover {
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  transform: translateY(-1px);
}

.card-done {
  opacity: 0.6;
  filter: grayscale(0.8);
  transition:
    opacity 0.2s,
    filter 0.2s;
}

.card-done:hover {
  opacity: 1;
  filter: grayscale(0);
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 0.25rem;
}

.card-id {
  font-size: 0.7rem;
  color: var(--text-muted);
  font-weight: 600;
}

.card-title {
  margin: 0 0 0.5rem 0;
  font-size: 0.95rem;
  line-height: 1.3;
  color: var(--text-primary);
  display: -webkit-box;
  -webkit-line-clamp: 2;
  line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.card-meta {
  display: flex;
  flex-wrap: wrap;
  gap: 0.25rem;
}

.epic-badge {
  font-size: 0.7rem;
  padding: 0.1rem 0.4rem;
  border-radius: 10px;
  font-weight: 600;
}

.epic-badge::before {
  content: "@";
}

.checklist-progress {
  display: flex;
  align-items: center;
  gap: 0.2rem;
  font-size: 0.7rem;
  color: var(--text-muted);
  font-weight: 600;
}

.checklist-complete {
  color: var(--p-success-color, #22c55e);
}

.tag-pill {
  font-size: 0.7rem;
  padding: 0.1rem 0.4rem;
  border-radius: 10px;
  background: var(--bg-secondary);
  color: var(--text-secondary);
  border: 1px solid var(--border-color);
}

.tag-pill::before {
  content: "#";
}
</style>
