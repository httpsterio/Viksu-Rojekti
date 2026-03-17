<script setup lang="ts">
import { ref, watch } from 'vue'
import { useBoard } from '@/composables/useBoard'
import type { BoardConfig, Epic } from '@/types'
import Dialog from 'primevue/dialog'
import Button from 'primevue/button'
import InputText from 'primevue/inputtext'
import Tag from 'primevue/tag'

const { config, saveBoardConfig } = useBoard()

const visible = ref(false)
const localConfig = ref<BoardConfig | null>(null)

const open = () => {
  if (config.value) {
    localConfig.value = JSON.parse(JSON.stringify(config.value))
    visible.value = true
  }
}

defineExpose({ open })

const handleSave = async () => {
  if (localConfig.value) {
    await saveBoardConfig(localConfig.value)
    visible.value = false
  }
}

const addLane = () => {
  localConfig.value?.lanes.push('new-lane')
}

const removeLane = (index: number) => {
  localConfig.value?.lanes.splice(index, 1)
}

const addEpic = () => {
  localConfig.value?.epics.push({
    id: `epic-${Date.now()}`,
    name: 'New Epic',
    color: '#3b82f6'
  })
}

const removeEpic = (index: number) => {
  localConfig.value?.epics.splice(index, 1)
}

const newTag = ref('')
const addTag = () => {
  if (newTag.value && !localConfig.value?.tags.includes(newTag.value)) {
    localConfig.value?.tags.push(newTag.value)
    newTag.value = ''
  }
}

const removeTag = (tag: string) => {
  if (localConfig.value) {
    localConfig.value.tags = localConfig.value.tags.filter(t => t !== tag)
  }
}
</script>

<template>
  <Dialog v-model:visible="visible" modal header="Board Settings" class="settings-modal">
    <div v-if="localConfig" class="settings-grid">
      <section>
        <label>Board Name</label>
        <InputText v-model="localConfig.name" fluid />
      </section>

      <section>
        <div class="section-header">
          <label>Lanes</label>
          <Button icon="pi pi-plus" size="small" text rounded @click="addLane" />
        </div>
        <div class="list-editor">
          <div v-for="(lane, index) in localConfig.lanes" :key="index" class="list-item">
            <InputText v-model="localConfig.lanes[index]" size="small" />
            <Button icon="pi pi-trash" severity="danger" text rounded size="small" @click="removeLane(index)" />
          </div>
        </div>
      </section>

      <section>
        <div class="section-header">
          <label>Epics</label>
          <Button icon="pi pi-plus" size="small" text rounded @click="addEpic" />
        </div>
        <div class="list-editor">
          <div v-for="(epic, index) in localConfig.epics" :key="epic.id" class="list-item epic-item">
            <input type="color" v-model="localConfig.epics[index].color" class="color-picker" />
            <InputText v-model="localConfig.epics[index].name" size="small" placeholder="Epic Name" />
            <InputText v-model="localConfig.epics[index].id" size="small" placeholder="ID (slug)" />
            <Button icon="pi pi-trash" severity="danger" text rounded size="small" @click="removeEpic(index)" />
          </div>
        </div>
      </section>

      <section>
        <label>Tags</label>
        <div class="tag-input">
          <InputText v-model="newTag" placeholder="Add tag..." size="small" @keyup.enter="addTag" />
          <Button icon="pi pi-plus" size="small" @click="addTag" :disabled="!newTag" />
        </div>
        <div class="tag-display">
          <Tag v-for="tag in localConfig.tags" :key="tag" :value="tag" closable @close="removeTag(tag)" />
        </div>
      </section>
    </div>

    <template #footer>
      <Button label="Cancel" text @click="visible = false" />
      <Button label="Save Changes" @click="handleSave" />
    </template>
  </Dialog>
</template>

<style scoped>
.settings-modal {
  width: 90vw;
  max-width: 600px;
}

.settings-grid {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  padding: 1rem 0;
}

section label {
  display: block;
  font-weight: 600;
  font-size: 0.85rem;
  color: var(--text-secondary);
  margin-bottom: 0.5rem;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.list-editor {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  max-height: 200px;
  overflow-y: auto;
  padding-right: 0.5rem;
}

.list-item {
  display: flex;
  gap: 0.5rem;
  align-items: center;
}

.list-item .p-inputtext {
  flex: 1;
}

.epic-item .p-inputtext {
  flex: 2;
}

.epic-item .p-inputtext:last-of-type {
  flex: 1;
}

.color-picker {
  width: 30px;
  height: 30px;
  padding: 0;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  background: none;
}

.tag-input {
  display: flex;
  gap: 0.5rem;
  margin-bottom: 0.5rem;
}

.tag-input .p-inputtext {
  flex: 1;
}

.tag-display {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
}
</style>
