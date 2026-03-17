<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useBoard } from '@/composables/useBoard'
import type { Card } from '@/types'
import Dialog from 'primevue/dialog'
import Button from 'primevue/button'
import InputText from 'primevue/inputtext'
import Select from 'primevue/select'
import Tag from 'primevue/tag'
import { useConfirm } from 'primevue/useconfirm'
import { MdEditor, MdPreview } from 'md-editor-v3'
import 'md-editor-v3/lib/style.css'

const { 
  config, 
  isCreating, 
  editingCard, 
  createCard, 
  updateCard, 
  deleteCard,
  isDarkMode
} = useBoard()

const visible = computed({
  get: () => isCreating.value || !!editingCard.value,
  set: (val) => {
    if (!val) {
      isCreating.value = false
      editingCard.value = null
    }
  }
})

const isNew = computed(() => isCreating.value)

const card = ref<Partial<Card>>({
  title: '',
  status: '',
  epic: null,
  tags: [],
  priority: 'medium',
  body: ''
})

const descriptionTab = ref<'view' | 'edit'>('view')
const confirm = useConfirm()

const editorToolbars = [
  'bold',
  'italic',
  'strikeThrough',
  '-',
  'title',
  'unorderedList',
  'orderedList',
  'task',
  '-',
  'code',
  'codeRow',
  'link',
  '-',
  'revoke',
  'next'
]

watch(visible, (val) => {
  if (val) {
    if (editingCard.value) {
      card.value = { ...editingCard.value }
      descriptionTab.value = 'view'
    } else {
      card.value = {
        title: '',
        status: config.value?.lanes[0] || '',
        epic: null,
        tags: [],
        priority: 'medium',
        body: ''
      }
      descriptionTab.value = 'edit'
    }
  }
})

const handleSave = async () => {
  if (!card.value.title) return

  if (isNew.value) {
    await createCard(card.value)
  } else {
    await updateCard(card.value as Card)
  }
  visible.value = false
}

const handleDelete = () => {
  if (card.value.id) {
    confirm.require({
      message: `Are you sure you want to delete ${card.value.id}?`,
      header: 'Delete Confirmation',
      icon: 'pi pi-exclamation-triangle',
      acceptProps: {
        label: 'Delete',
        severity: 'danger'
      },
      rejectProps: {
        label: 'Cancel',
        severity: 'secondary',
        text: true
      },
      accept: async () => {
        if (card.value.id) {
          await deleteCard(card.value.id)
          visible.value = false
        }
      }
    })
  }
}

const toggleTag = (tag: string) => {
  const index = card.value.tags?.indexOf(tag) ?? -1
  if (index === -1) {
    card.value.tags?.push(tag)
  } else {
    card.value.tags?.splice(index, 1)
  }
}
</script>

<template>
  <Dialog 
    v-model:visible="visible" 
    modal 
    :header="isNew ? 'Create New Card' : `Edit Card: ${card.id}`" 
    class="card-modal"
  >
    <div class="modal-grid">
      <div class="main-fields">
        <div class="field">
          <label>Title</label>
          <InputText v-model="card.title" placeholder="What needs to be done?" fluid autofocus />
        </div>

        <div class="row">
          <div class="field">
            <label>Status</label>
            <Select v-model="card.status" :options="config?.lanes" placeholder="Select Status" fluid />
          </div>
          <div class="field">
            <label>Priority</label>
            <Select v-model="card.priority" :options="config?.priorities" placeholder="Select Priority" fluid />
          </div>
        </div>

        <div class="field">
          <label>Epic</label>
          <Select 
            v-model="card.epic" 
            :options="config?.epics" 
            optionLabel="name" 
            optionValue="id" 
            placeholder="No Epic" 
            showClear 
            fluid 
          />
        </div>

        <div class="field">
          <label>Tags</label>
          <div class="tag-selector">
            <Tag 
              v-for="tag in config?.tags" 
              :key="tag" 
              :value="tag"
              :class="{ 'tag-selected': card.tags?.includes(tag) }"
              @click="toggleTag(tag)"
            />
          </div>
        </div>
      </div>

      <div class="description-section">
        <div class="description-header">
          <label>Description</label>
          <div class="tabs">
            <Button 
              label="View" 
              size="small" 
              :text="descriptionTab !== 'view'" 
              @click="descriptionTab = 'view'" 
            />
            <Button 
              label="Edit" 
              size="small" 
              :text="descriptionTab !== 'edit'" 
              @click="descriptionTab = 'edit'" 
            />
          </div>
        </div>

        <div class="description-content">
          <MdPreview 
            v-if="descriptionTab === 'view'" 
            v-model="card.body" 
            :theme="isDarkMode ? 'dark' : 'light'" 
          />
          <MdEditor 
            v-else 
            v-model="card.body" 
            :toolbars="editorToolbars" 
            :preview="false" 
            :theme="isDarkMode ? 'dark' : 'light'" 
          />
        </div>
      </div>
    </div>

    <template #footer>
      <div class="footer-buttons">
        <Button 
          v-if="!isNew" 
          label="Delete" 
          severity="danger" 
          text 
          @click="handleDelete" 
        />
        <div class="right-buttons">
          <Button label="Cancel" text @click="visible = false" />
          <Button label="Save" @click="handleSave" :disabled="!card.title" />
        </div>
      </div>
    </template>
  </Dialog>
</template>

<style scoped>
.card-modal {
  width: 90vw;
  max-width: 800px;
}

.modal-grid {
  display: grid;
  grid-template-columns: 300px 1fr;
  gap: 2rem;
  padding-top: 1rem;
}

@media (max-width: 768px) {
  .modal-grid {
    grid-template-columns: 1fr;
  }
}

.field {
  margin-bottom: 1.25rem;
}

.field label {
  display: block;
  font-weight: 600;
  margin-bottom: 0.5rem;
  font-size: 0.85rem;
  color: var(--text-secondary);
}

.row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1rem;
}

.tag-selector {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
}

.tag-selector .p-tag {
  cursor: pointer;
  opacity: 0.5;
  transition: opacity 0.2s;
}

.tag-selector .p-tag.tag-selected {
  opacity: 1;
}

.description-section {
  display: flex;
  flex-direction: column;
  height: 500px;
}

.description-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 0.5rem;
}

.description-header label {
  font-weight: 600;
  font-size: 0.85rem;
  color: var(--text-secondary);
}

.description-content {
  flex: 1;
  border: 1px solid var(--border-color);
  border-radius: var(--card-radius);
  overflow: hidden;
  background: var(--bg-card);
}

.footer-buttons {
  display: flex;
  justify-content: space-between;
  width: 100%;
}

.right-buttons {
  display: flex;
  gap: 0.5rem;
}

:deep(.md-editor), :deep(.md-preview) {
  height: 100% !important;
}
</style>
