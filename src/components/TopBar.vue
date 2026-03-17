<script setup lang="ts">
import { useBoard } from '@/composables/useBoard'
import Button from 'primevue/button'
import Select from 'primevue/select'
import InputText from 'primevue/inputtext'

const { 
  config, 
  activeFilters, 
  currentView, 
  isCreating,
  toggleDarkMode,
  isDarkMode 
} = useBoard()

const views = [
  { label: 'Board', value: 'board', icon: 'pi pi-th-large' },
  { label: 'Epics', value: 'epics', icon: 'pi pi-list' }
]
</script>

<template>
  <div class="top-bar" v-if="config">
    <div class="left">
      <h2 class="board-name">{{ config.name }}</h2>
      <div class="view-toggle">
        <Button 
          v-for="view in views" 
          :key="view.value"
          :icon="view.icon"
          :label="view.label"
          :class="{ 'p-button-secondary': currentView !== view.value }"
          @click="currentView = (view.value as 'board' | 'epics')"
          size="small"
        />
      </div>
    </div>

    <div class="center">
      <div class="filters">
        <Select 
          v-model="activeFilters.epic" 
          :options="config.epics" 
          optionLabel="name" 
          optionValue="id" 
          placeholder="All Epics" 
          showClear 
          size="small"
        />
        <Select 
          v-model="activeFilters.tag" 
          :options="config.tags" 
          placeholder="All Tags" 
          showClear 
          size="small"
        />
        <Select 
          v-model="activeFilters.priority" 
          :options="config.priorities" 
          placeholder="All Priorities" 
          showClear 
          size="small"
        />
        <span class="p-input-icon-left">
          <i class="pi pi-search" />
          <InputText v-model="activeFilters.search" placeholder="Search..." size="small" />
        </span>
      </div>
    </div>

    <div class="right">
      <Button 
        :icon="isDarkMode ? 'pi pi-sun' : 'pi pi-moon'" 
        @click="toggleDarkMode" 
        rounded 
        text 
      />
      <Button icon="pi pi-cog" rounded text />
      <Button label="New Ticket" icon="pi pi-plus" @click="isCreating = true" />
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
}

.left, .center, .right {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.board-name {
  margin: 0;
  font-size: 1.2rem;
  white-space: nowrap;
}

.view-toggle {
  display: flex;
  gap: 0.25rem;
}

.filters {
  display: flex;
  gap: 0.5rem;
}
</style>
