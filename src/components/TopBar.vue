<script setup lang="ts">
import { useBoard } from "@/composables/useBoard"
import { ref, computed } from "vue"
import Button from "primevue/button"
import Select from "primevue/select"
import InputText from "primevue/inputtext"
import ButtonGroup from "primevue/buttongroup"
import InputGroup from "primevue/inputgroup"
import InputGroupAddon from "primevue/inputgroupaddon"
import Popover from "primevue/popover"

const { config, activeFilters, currentView, isCreating, toggleDarkMode } = useBoard()

defineEmits(["open-settings"])

const filterPanel = ref()

const views = [
  { label: "Board", value: "board", icon: "pi pi-th-large" },
  { label: "Epics", value: "epics", icon: "pi pi-list" },
]

const activeFilterCount = computed(() => {
  let count = 0
  if (activeFilters.value.epic) count++
  if (activeFilters.value.tag) count++
  if (activeFilters.value.priority) count++
  return count
})

const priorityFilterOptions = computed(() => {
  if (!config.value) return []
  return config.value.priorities.map((p, i) => ({
    label: p.name,
    value: i + 1,
    color: p.color,
  }))
})

const toggleFilters = (event: Event) => {
  filterPanel.value.toggle(event)
}

const clearAllFilters = () => {
  activeFilters.value.epic = null
  activeFilters.value.tag = null
  activeFilters.value.priority = null
  activeFilters.value.search = ""
}
</script>

<template>
  <div v-if="config" class="top-bar">
    <div class="left">
      <ButtonGroup>
        <Button
          v-for="view in views"
          :key="view.value"
          :icon="view.icon"
          :label="view.label"
          :class="{ 'p-button-secondary': currentView !== view.value }"
          size="small"
          @click="currentView = view.value as 'board' | 'epics'"
        />
      </ButtonGroup>
    </div>

    <div class="center">
      <Button
        icon="pi pi-filter"
        :label="activeFilterCount > 0 ? `Filters (${activeFilterCount})` : 'Filters'"
        :severity="activeFilterCount > 0 ? undefined : 'secondary'"
        size="small"
        outlined
        @click="toggleFilters"
      />
      <Button
        v-if="activeFilterCount > 0"
        icon="pi pi-filter-slash"
        size="small"
        text
        severity="secondary"
        @click="clearAllFilters"
      />
      <Popover ref="filterPanel">
        <div class="filter-panel">
          <Select
            v-model="activeFilters.epic"
            :options="config.epics"
            option-label="name"
            option-value="name"
            placeholder="All Epics"
            show-clear
            size="small"
            fluid
          />
          <Select
            v-model="activeFilters.tag"
            :options="config.tags"
            option-label="name"
            option-value="name"
            placeholder="All Tags"
            show-clear
            size="small"
            fluid
          />
          <Select
            v-model="activeFilters.priority"
            :options="priorityFilterOptions"
            option-label="label"
            option-value="value"
            placeholder="All Priorities"
            show-clear
            size="small"
            fluid
          >
            <template #value="{ value, placeholder }">
              <div v-if="value" style="display: flex; align-items: center; gap: 0.5rem">
                <span
                  style="width: 0.6rem; height: 0.6rem; border-radius: 50%; display: inline-block"
                  :style="{
                    backgroundColor: priorityFilterOptions.find((o) => o.value === value)?.color,
                  }"
                ></span>
                <span>{{ priorityFilterOptions.find((o) => o.value === value)?.label }}</span>
              </div>
              <span v-else>{{ placeholder }}</span>
            </template>
            <template #option="{ option }">
              <div style="display: flex; align-items: center; gap: 0.5rem">
                <span
                  style="width: 0.6rem; height: 0.6rem; border-radius: 50%; display: inline-block"
                  :style="{ backgroundColor: option.color }"
                ></span>
                <span>{{ option.label }}</span>
              </div>
            </template>
          </Select>
        </div>
      </Popover>
      <InputGroup class="search-group">
        <InputText v-model="activeFilters.search" placeholder="Search..." size="small" />
        <InputGroupAddon>
          <Button
            icon="pi pi-times"
            text
            severity="secondary"
            size="small"
            @click="activeFilters.search = ''"
          />
        </InputGroupAddon>
      </InputGroup>
    </div>

    <div class="right">
      <ButtonGroup>
        <Button icon="pi pi-moon" text @click="toggleDarkMode" />
        <Button icon="pi pi-cog" text @click="$emit('open-settings')" />
      </ButtonGroup>
      <Button label="New Card" size="small" icon="pi pi-plus" @click="isCreating = true" />
    </div>
  </div>
</template>

<style scoped>
.top-bar {
  height: var(--topbar-height);
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 1rem;
  background: var(--bg-primary);
  border-bottom: 1px solid var(--border-color);
  gap: 0.75rem;
}

.left,
.center,
.right {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.center {
  flex: 1;
  justify-content: center;
}

.search-group {
  max-width: 220px;
}

.filter-panel {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  padding: 0.25rem;
  min-width: 200px;
}
</style>
