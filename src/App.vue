<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useBoard } from './composables/useBoard'
import Toast from 'primevue/toast'
import Button from 'primevue/button'
import InputText from 'primevue/inputtext'
import ProgressSpinner from 'primevue/progressspinner'
import TopBar from './components/TopBar.vue'
import BoardView from './components/BoardView.vue'
import TicketModal from './components/TicketModal.vue'

const { config, isLoading, needsInit, loadBoard, initBoard, currentView } = useBoard()

const initName = ref('')
const initPrefix = ref('')

onMounted(() => {
  loadBoard()
})

const handleInit = () => {
  if (initName.value && initPrefix.value) {
    initBoard(initName.value, initPrefix.value)
  }
}
</script>

<template>
  <div class="app-container">
    <Toast />

    <div v-if="isLoading" class="loading-overlay">
      <ProgressSpinner />
    </div>

    <div v-else-if="needsInit" class="init-container">
      <div class="init-card">
        <h1>Welcome to Rojekti</h1>
        <p>No board found in this directory. Initialize a new one?</p>
        
        <div class="field">
          <label for="name">Board Name</label>
          <InputText id="name" v-model="initName" placeholder="My Project" fluid />
        </div>
        
        <div class="field">
          <label for="prefix">Ticket Prefix</label>
          <InputText id="prefix" v-model="initPrefix" placeholder="ROJ" fluid />
        </div>
        
        <Button label="Initialize Board" @click="handleInit" :disabled="!initName || !initPrefix" />
      </div>
    </div>

    <template v-else-if="config">
      <TopBar />
      <main class="main-content">
        <BoardView v-if="currentView === 'board'" />
        <div v-else class="epic-view-placeholder">
          <h2>Epic View Coming Soon (Phase 5)</h2>
        </div>
      </main>
      <TicketModal />
    </template>
  </div>
</template>

<style>
.app-container {
  height: 100vh;
  display: flex;
  flex-direction: column;
}

.main-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.loading-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  display: flex;
  justify-content: center;
  align-items: center;
  background: var(--bg-primary);
  z-index: 1000;
}

.init-container {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100%;
  background: var(--bg-secondary);
}

.init-card {
  background: var(--bg-card);
  padding: 2rem;
  border-radius: var(--card-radius);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  width: 100%;
  max-width: 400px;
}

.field {
  margin-bottom: 1.5rem;
}

.field label {
  display: block;
  margin-bottom: 0.5rem;
  font-weight: 600;
}

.epic-view-placeholder {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100%;
}
</style>
