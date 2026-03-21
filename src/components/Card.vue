<script setup lang="ts">
import { computed } from "vue"
import { useBoard } from "@/composables/useBoard"
import { useColorContrast } from "@/composables/useColorContrast"
import type { Card } from "@/types"

const props = defineProps<{
  card: Card
}>()

const { config, editingCard } = useBoard()
const { contrastColor } = useColorContrast()

const epic = computed(() => config.value?.epics.find((e) => e.name === props.card.epic))

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
    :class="card.priority === 0 ? 'priority-0' : ''"
    :style="
      card.priority > 0 && config
        ? { borderLeftColor: config.priorities[card.priority - 1]?.color }
        : {}
    "
    :data-card-id="card.id"
    @click="editingCard = card"
  >
    <div class="card-header">
      <span class="card-id">{{ card.id }}</span>
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
      <span v-for="tagName in card.tags" :key="tagName" class="tag-pill" :style="getTagStyle(tagName)">
        {{ getTag(tagName)?.name || tagName }}
      </span>
    </div>
  </div>
</template>

<style scoped>
.card {
  background: var(--bg-card);
  border-left: 4px solid transparent;
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

.card-header {
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

.tag-pill {
  font-size: 0.7rem;
  padding: 0.1rem 0.4rem;
  border-radius: 10px;
  background: var(--bg-secondary);
  color: var(--text-secondary);
  border: 1px solid var(--border-color);
}
</style>
