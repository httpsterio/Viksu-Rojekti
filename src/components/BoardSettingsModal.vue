<script setup lang="ts">
import { ref } from 'vue'
import { useBoard } from '@/composables/useBoard'
import type { BoardConfig } from '@/types'
import Dialog from 'primevue/dialog'
import Button from 'primevue/button'
import InputText from 'primevue/inputtext'
import InputGroup from 'primevue/inputgroup'
import InputGroupAddon from 'primevue/inputgroupaddon'
import { useConfirm } from 'primevue/useconfirm'
import Sortable from 'sortablejs'

const { config, saveBoardConfig } = useBoard()
const confirm = useConfirm()

const visible = ref(false)
const localConfig = ref<BoardConfig | null>(null)
const epicsContainer = ref<HTMLElement | null>(null)
const tagsContainer = ref<HTMLElement | null>(null)
let epicsSortable: Sortable | null = null
let tagsSortable: Sortable | null = null

const open = () => {
  if (config.value) {
    localConfig.value = JSON.parse(JSON.stringify(config.value))
    visible.value = true
  }
}

const onShow = () => {
  epicsSortable?.destroy()
  tagsSortable?.destroy()
  epicsSortable = initSortable(epicsContainer.value, 'epics')
  tagsSortable = initSortable(tagsContainer.value, 'tags')
}

const initSortable = (el: HTMLElement | null, list: 'epics' | 'tags') => {
  if (el && localConfig.value) {
    return new Sortable(el, {
      handle: '.drag-handle',
      animation: 150,
      forceFallback: true,
      onEnd: (evt) => {
        if (evt.oldIndex !== undefined && evt.newIndex !== undefined && localConfig.value) {
          const item = localConfig.value[list].splice(evt.oldIndex, 1)[0]
          localConfig.value[list].splice(evt.newIndex, 0, item as any)
        }
      }
    })
  }
  return null
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
  confirm.require({
    message: 'Are you sure you want to delete this epic?',
    header: 'Delete Epic',
    icon: 'pi pi-exclamation-triangle',
    acceptClass: 'p-button-danger',
    accept: () => {
      localConfig.value?.epics.splice(index, 1)
    }
  })
}

const addTag = () => {
  localConfig.value?.tags.push({
    id: `tag-${Date.now()}`,
    name: 'New Tag',
    color: '#10b981'
  })
}

const removeTag = (index: number) => {
  confirm.require({
    message: 'Are you sure you want to delete this tag?',
    header: 'Delete Tag',
    icon: 'pi pi-exclamation-triangle',
    acceptClass: 'p-button-danger',
    accept: () => {
      localConfig.value?.tags.splice(index, 1)
    }
  })
}
</script>

<template>
  <Dialog v-model:visible="visible" modal header="Board Settings" class="settings-modal" @show="onShow"
    :dismissableMask="true" :draggable="false">
    <div v-if="localConfig" class="settings-layout">
      <div class="settings-column">
        <section>
          <label>Board Name</label>
          <InputText v-model="localConfig.name" fluid />
        </section>

        <section>
          <div class="section-header">
            <label>Status</label>
          </div>
          <div class="list-editor">
            <div v-for="(lane, index) in localConfig.lanes" :key="index" class="list-item">
              <InputGroup>
                <InputText v-model="localConfig.lanes[index]" size="small" />
                <InputGroupAddon>
                  <Button icon="pi pi-trash" text severity="danger" size="small" @click="removeLane(index)" />
                </InputGroupAddon>
              </InputGroup>
            </div>
            <Button icon="pi pi-plus" label="Add Status" size="small" class="add-btn" @click="addLane" />
          </div>
        </section>

        <section>
          <div class="section-header">
            <label>Priorities</label>
          </div>
          <div class="list-editor">
            <div v-for="(priority, index) in localConfig.priorities" :key="index" class="list-item">
              <InputGroup>
                <InputText v-model="localConfig.priorities[index]" size="small" />
              </InputGroup>
            </div>
          </div>
        </section>
      </div>

      <div class="settings-column">
        <section>
          <div class="section-header">
            <label>Epics</label>
          </div>
          <div class="list-editor" ref="epicsContainer">
            <div v-for="(epic, index) in localConfig.epics" :key="epic.id" class="list-item">
              <InputGroup>
                <InputGroupAddon class="drag-handle">
                  <i class="pi pi-bars"></i>
                </InputGroupAddon>
                <InputGroupAddon class="color-addon">
                  <input type="color" v-model="epic.color" class="color-swatch" />
                </InputGroupAddon>
                <InputText v-model="epic.name" placeholder="Epic name" />
                <InputGroupAddon>
                  <Button icon="pi pi-times" text severity="secondary" @click="removeEpic(index)" />
                </InputGroupAddon>
              </InputGroup>
            </div>
            <Button icon="pi pi-plus" label="Add Epic" size="small" class="add-btn" @click="addEpic" />
          </div>
        </section>

        <section>
          <div class="section-header">
            <label>Tags</label>
          </div>
          <div class="list-editor" ref="tagsContainer">
            <div v-for="(tag, index) in localConfig.tags" :key="tag.id" class="list-item">
              <InputGroup>
                <InputGroupAddon class="drag-handle">
                  <i class="pi pi-bars"></i>
                </InputGroupAddon>
                <InputGroupAddon class="color-addon">
                  <input type="color" v-model="tag.color" class="color-swatch" />
                </InputGroupAddon>
                <InputText v-model="tag.name" placeholder="Tag name" />
                <InputGroupAddon>
                  <Button icon="pi pi-times" text severity="secondary" @click="removeTag(index)" />
                </InputGroupAddon>
              </InputGroup>
            </div>
            <Button icon="pi pi-plus" label="Add Tag" size="small" class="add-btn" @click="addTag" />
          </div>
        </section>
      </div>
    </div>

    <template #footer>
      <Button label="Cancel" text @click="visible = false" />
      <Button label="Save Changes" @click="handleSave" />
    </template>
  </Dialog>
</template>

<style scoped>

.settings-layout {
  display: grid;
  grid-template-columns: 1fr;
  gap: 2rem;
  padding: 0.5rem 0;
  max-width: 850px;
  margin: 0 auto;
}

@media (min-width: 600px) {
  .settings-layout {
    grid-template-columns: 1fr 1fr;
  }
}

.settings-column {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  max-width: 500px;
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
  overflow-y: auto;
  padding-right: 0.5rem;
}

.list-item {
  display: block;
}

.drag-handle {
  cursor: grab;
}

.drag-handle i {
  color: var(--text-muted);
}

.color-addon {
  padding: 0.4rem;
  display: flex;
  align-items: center;
  justify-content: center;
}

.color-swatch {
  width: 1.8rem;
  height: 1.8rem;
  padding: 0;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  cursor: pointer;
  background: none;
  appearance: none;
  -webkit-appearance: none;
}

.color-swatch::-webkit-color-swatch-wrapper {
  padding: 0;
}

.color-swatch::-webkit-color-swatch {
  border: none;
  border-radius: 3px;
}

.color-swatch::-moz-color-swatch {
  border: none;
  border-radius: 3px;
}

.add-btn {
  max-width: 8rem;
  align-self: center;
  margin-top: 0.25rem;
}
</style>