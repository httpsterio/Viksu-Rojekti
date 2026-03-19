<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useBoard } from '@/composables/useBoard'
import type { Card } from '@/types'
import Dialog from 'primevue/dialog'
import Button from 'primevue/button'
import InputText from 'primevue/inputtext'
import Select from 'primevue/select'
import MultiSelect from 'primevue/multiselect'
import Tag from 'primevue/tag'
import { useConfirm } from 'primevue/useconfirm'
import { MdEditor, MdPreview } from 'md-editor-v3'
import type { ToolbarNames } from 'md-editor-v3'

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

const editorToolbars: ToolbarNames[] = [
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
  // '-',
  // 'revoke',
  // 'next'
]

watch(visible, (val) => {
  if (val) {
    if (editingCard.value) {
      card.value = { ...editingCard.value }
      if (!card.value.tags) card.value.tags = []
      descriptionTab.value = 'view'
    } else {
      card.value = {
        title: '',
        status: config.value?.statuses[0]?.id || '',
        epic: null,
        tags: [],
        priority: 'medium',
        body: ''
      }
      descriptionTab.value = 'edit'
    }
  }
})

const isSaving = ref(false)

const handleSave = async () => {
  if (!card.value.title || isSaving.value) return

  isSaving.value = true
  try {
    if (isNew.value) {
      await createCard(card.value)
    } else {
      await updateCard(card.value as Card)
    }
    visible.value = false
  } finally {
    isSaving.value = false
  }
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

const getTag = (id: string) => config.value?.tags.find(t => t.id === id)

const removeTag = (tag: string) => {
  if (card.value.tags) {
    card.value.tags = card.value.tags.filter(t => t !== tag)
  }
}
</script>

<template>
  <Dialog 
    v-model:visible="visible" 
    modal 
    :header="isNew ? 'Create New Card' : `Edit Card: ${card.id}`" 
    class="card-modal"
    :draggable="false"
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
            <Select 
              v-model="card.status" 
              :options="config?.statuses" 
              optionLabel="name"
              optionValue="id"
              placeholder="Select Status" 
              fluid 
            />
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
          <MultiSelect 
            v-model="card.tags" 
            :options="config?.tags" 
            optionLabel="name"
            optionValue="id"
            placeholder="Select Tags" 
            display="chip" 
            fluid 
            :maxSelectedLabels="3"
            :showSelectAll="false"
          />
          <div class="selected-tags" v-if="card.tags && card.tags.length > 0">
            <Tag 
              v-for="tagId in card.tags" 
              :key="tagId" 
              :value="getTag(tagId)?.name || tagId"
              :style="{ backgroundColor: getTag(tagId)?.color }"
              class="removable-tag"
              icon="pi pi-times"
              @click="removeTag(tagId)"
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
            previewTheme="github"
            language="en-US"
            :theme="isDarkMode ? 'dark' : 'light'"
          />
          <MdEditor 
            v-else 
            v-model="card.body" 
            :toolbars="editorToolbars" 
            :preview="false"
            language="en-US"
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
          <Button label="Cancel" text @click="visible = false" :disabled="isSaving" />
          <Button label="Save" @click="handleSave" :disabled="!card.title" :loading="isSaving" />
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

.selected-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
  margin-top: 0.5rem;
}

.removable-tag {
  cursor: pointer;
  transition: opacity 0.2s;
}

.removable-tag:hover {
  opacity: 0.8;
}

:deep(.md-editor), :deep(.md-preview) {
  height: 100% !important;
  border: none !important;
}

:deep(.md-editor-content) {
  border: none !important;
}

:deep(.md-editor-preview-wrapper) {
  padding: 10px;
}

:deep(.md-editor-preview) {
  padding: 0;
}
</style>
