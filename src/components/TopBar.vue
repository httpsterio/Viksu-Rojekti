<script setup lang="ts">
import { useBoard } from '@/composables/useBoard'
import Button from 'primevue/button'
import Select from 'primevue/select'
import InputText from 'primevue/inputtext'
import ButtonGroup from 'primevue/buttongroup'
import InputGroup from 'primevue/inputgroup';
import InputGroupAddon from 'primevue/inputgroupaddon';

const { 
  config, 
  activeFilters, 
  currentView, 
  isCreating,
  toggleDarkMode,
  isDarkMode 
} = useBoard()

defineEmits(['open-settings'])

const views = [
  { label: 'Board', value: 'board', icon: 'pi pi-th-large' },
  { label: 'Epics', value: 'epics', icon: 'pi pi-list' }
]
</script>

<template>
  <div class="top-bar" v-if="config">
    <div class="left">
      <!-- <h2 class="board-name">{{ config.name }}</h2> -->
      <div class="view-toggle">
        <ButtonGroup>
        <Button 
          v-for="view in views" 
          :key="view.value"
          :icon="view.icon"
          :label="view.label"
          :class="{ 'p-button-secondary': currentView !== view.value }"
          @click="currentView = (view.value as 'board' | 'epics')"
          size="small"
        />
        </ButtonGroup>
      </div>
    </div>

    <div class="center">
      <div class="filters">
        <Select v-model="activeFilters.epic" :options="config.epics" optionLabel="name" optionValue="id"
          placeholder="All Epics" showClear size="small" style="min-width: 160px" />
        <Select v-model="activeFilters.tag" :options="config.tags" optionLabel="name" optionValue="id"
          placeholder="All Tags" showClear size="small" style="min-width: 140px"  />
        <Select 
          v-model="activeFilters.priority" 
          :options="config.priorities" 
          placeholder="All Priorities" 
          showClear 
          size="small"
        />
        <InputGroup>
          <!-- <InputGroupAddon>
            <i class="pi pi-search" />
          </InputGroupAddon> -->
          <InputText v-model="activeFilters.search" placeholder="Search..." size="small" />
          <InputGroupAddon>
            <Button icon="pi pi-times" text severity="secondary" size="small" @click="activeFilters.search = ''" />
          </InputGroupAddon>
        </InputGroup>
        

      </div>
    </div>

    <div class="right">
      <ButtonGroup>
      <!-- <Button 
        :icon="isDarkMode ? 'pi pi-sun' : 'pi pi-moon'" 
        @click="toggleDarkMode" 
         text
      /> -->
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
