<script setup lang="ts">
import { ref, computed, watch } from "vue"
import { useBoard } from "@/composables/useBoard"
import type { Card } from "@/types"
import Dialog from "primevue/dialog"
import Button from "primevue/button"
import InputText from "primevue/inputtext"
import Select from "primevue/select"
import MultiSelect from "primevue/multiselect"
import Tag from "primevue/tag"
import Chip from "primevue/chip"
import ButtonGroup from "primevue/buttongroup"
import { useConfirm } from "primevue/useconfirm"
import { MdEditor, MdPreview } from "md-editor-v3"
import type { ToolbarNames } from "md-editor-v3"
import { useColorContrast } from "@/composables/useColorContrast"

const { config, isCreating, editingCard, createCard, updateCard, deleteCard, isDark } =
  useBoard()

const { contrastColor } = useColorContrast()

const visible = computed({
  get: () => isCreating.value || !!editingCard.value,
  set: (val) => {
    if (!val) {
      isCreating.value = false
      editingCard.value = null
    }
  },
})

const isNew = computed(() => isCreating.value)

const getInitialCard = (): Card => ({
  id: "",
  title: "",
  status: "",
  epic: null,
  tags: [],
  priority: 0,
  position: 0,
  created: "",
  body: "",
})

const card = ref<Card>(getInitialCard())

const priorityOptions = computed(() => {
  const options = [{ label: "None", value: 0, color: "transparent" }]
  if (config.value) {
    config.value.priorities.forEach((p, index) => {
      options.push({ label: p.name, value: index + 1, color: p.color })
    })
  }
  return options
})

const knownTags = computed({
  get: () => card.value.tags.filter((name) => config.value?.tags.some((t) => t.name === name)),
  set: (val) => {
    card.value.tags = [...val, ...orphanedTags.value]
  },
})

const orphanedTags = computed(() =>
  card.value.tags.filter((name) => !config.value?.tags.some((t) => t.name === name)),
)

const isEpicOrphaned = computed(
  () => card.value.epic && !config.value?.epics.some((e) => e.name === card.value.epic),
)

const descriptionTab = ref<"view" | "edit">("view")
const confirm = useConfirm()

const editorToolbars: ToolbarNames[] = [
  "bold",
  "italic",
  // "strikeThrough",
  "-",
  "title",
  "unorderedList",
  "orderedList",
  // "task",
  "-",
  "code",
  "codeRow",
  "link",
  // '-',
  // 'revoke',
  // 'next'
]

watch(visible, (val) => {
  if (val) {
    if (editingCard.value) {
      card.value = { ...editingCard.value }
      if (!card.value.tags) card.value.tags = []
      descriptionTab.value = "view"
    } else {
      card.value = {
        ...getInitialCard(),
        status: config.value?.statuses[0]?.name || "",
      }
      descriptionTab.value = "edit"
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
      await updateCard(card.value)
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
      header: "Delete Confirmation",
      icon: "pi pi-exclamation-triangle",
      acceptProps: {
        label: "Delete",
        severity: "danger",
      },
      rejectProps: {
        label: "Cancel",
        severity: "secondary",
        text: true,
      },
      accept: async () => {
        if (card.value.id) {
          await deleteCard(card.value.id)
          visible.value = false
        }
      },
    })
  }
}

const getTag = (name: string) => config.value?.tags.find((t) => t.name === name)
const getEpic = (name: string) => config.value?.epics.find((e) => e.name === name)

const removeTag = (name: string) => {
  if (card.value.tags) {
    card.value.tags = card.value.tags.filter((t) => t !== name)
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
              option-label="name"
              option-value="name"
              placeholder="Select Status"
              fluid
            />
          </div>
          <div class="field">
            <label>Priority</label>
            <Select
              v-model="card.priority"
              :options="priorityOptions"
              option-label="label"
              option-value="value"
              placeholder="Select Priority"
              fluid
            >
              <template #value="{ value, placeholder }">
                <div v-if="value !== undefined" class="priority-option">
                  <span
                    v-if="value > 0"
                    class="priority-swatch"
                    :style="{
                      backgroundColor: priorityOptions.find((o) => o.value === value)?.color,
                    }"
                  ></span>
                  <span>{{ priorityOptions.find((o) => o.value === value)?.label }}</span>
                </div>
                <span v-else>{{ placeholder }}</span>
              </template>
              <template #option="{ option }">
                <div class="priority-option">
                  <span
                    v-if="option.value > 0"
                    class="priority-swatch"
                    :style="{ backgroundColor: option.color }"
                  ></span>
                  <span>{{ option.label }}</span>
                </div>
              </template>
            </Select>
          </div>
        </div>

        <div class="field">
          <label>Epic</label>
          <Select
            v-model="card.epic"
            :options="config?.epics"
            option-label="name"
            option-value="name"
            placeholder="No Epic"
            show-clear
            fluid
          >
            <template #value="{ value }">
              <span
                v-if="value"
                class="colored-option"
                :style="{
                  backgroundColor: getEpic(value)?.color,
                  color: contrastColor(getEpic(value)?.color),
                }"
              >
                {{ getEpic(value)?.name || value }}
              </span>
              <span v-else class="p-placeholder">No Epic</span>
            </template>
            <template #option="{ option }">
              <span
                class="colored-option"
                :style="{ backgroundColor: option.color, color: contrastColor(option.color) }"
              >
                {{ option.name }}
              </span>
            </template>
          </Select>
          <div v-if="isEpicOrphaned" class="orphaned-info">
            <i class="pi pi-info-circle"></i>
            Current epic "{{ card.epic }}" is no longer in settings.
          </div>
        </div>

        <div class="field">
          <label>Tags</label>
          <MultiSelect
            v-model="knownTags"
            :options="config?.tags"
            option-label="name"
            option-value="name"
            placeholder="Select Tags"
            display="chip"
            fluid
            :max-selected-labels="3"
            :show-select-all="false"
          >
            <template #chip="{ value }">
              <span
                class="colored-chip"
                :style="{
                  backgroundColor: getTag(value)?.color,
                  color: contrastColor(getTag(value)?.color),
                }"
              >
                {{ getTag(value)?.name || value }}
              </span>
            </template>
            <template #option="{ option }">
              <span
                class="colored-option"
                :style="{ backgroundColor: option.color, color: contrastColor(option.color) }"
              >
                {{ option.name }}
              </span>
            </template>
          </MultiSelect>
          <div v-if="card.tags && card.tags.length > 0" class="selected-tags">
            <Tag
              v-for="tagName in knownTags"
              :key="tagName"
              :value="getTag(tagName)?.name || tagName"
              :style="{
                backgroundColor: getTag(tagName)?.color,
                color: contrastColor(getTag(tagName)?.color),
              }"
              class="removable-tag"
              icon="pi pi-times"
              @click="removeTag(tagName)"
            />
            <Tag
              v-for="tagName in orphanedTags"
              :key="tagName"
              :value="tagName"
              severity="secondary"
              class="removable-tag orphaned-tag"
              icon="pi pi-times"
              @click="removeTag(tagName)"
            />
          </div>
        </div>
      </div>

      <div class="description-section">
        <div class="description-header">
          <label>Description</label>
          <div class="tabs">
            <ButtonGroup>
              <Button
                label="View"
                size="small"
                icon="pi pi-eye"
                :text="descriptionTab !== 'view'"
                @click="descriptionTab = 'view'"
              />
              <Button
                label="Edit"
                size="small"
                icon="pi pi-pen-to-square"
                :text="descriptionTab !== 'edit'"
                @click="descriptionTab = 'edit'"
              />
            </ButtonGroup>
          </div>
        </div>

        <div class="description-content">
          <MdPreview
            v-if="descriptionTab === 'view'"
            v-model="card.body"
            preview-theme="github"
            language="en-US"
            :theme="isDark ? 'dark' : 'light'"
          />
          <MdEditor
            v-else
            v-model="card.body"
            :toolbars="editorToolbars"
            :preview="false"
            language="en-US"
            :theme="isDark ? 'dark' : 'light'"
          />
        </div>
      </div>
    </div>

    <template #footer>
      <div class="footer-buttons">
        <Button v-if="!isNew" label="Delete" severity="danger" text @click="handleDelete" />
        <div class="right-buttons">
          <Button label="Cancel" text :disabled="isSaving" @click="visible = false" />
          <Button label="Save" :disabled="!card.title" :loading="isSaving" @click="handleSave" />
        </div>
      </div>
    </template>
  </Dialog>
</template>

<style scoped>
.card-modal {
  width: 90vw;
  max-width: 20rem;
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
  padding: 1rem;
  min-width: 10rem;
  max-width: 52rem;
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

.orphaned-info {
  margin-top: 0.5rem;
  font-size: 0.75rem;
  color: var(--text-secondary);
  display: flex;
  align-items: center;
  gap: 0.25rem;
}

.orphaned-info i {
  color: var(--text-muted);
}

.orphaned-tag {
  opacity: 0.7;
}

.colored-option {
  display: inline-block;
  padding: 0.15rem 0.5rem;
  border-radius: 10px;
  font-size: 0.85rem;
}

.colored-chip {
  display: inline-block;
  padding: 0.15rem 0.5rem;
  border-radius: 10px;
  font-size: 0.75rem;
}

:deep(.md-editor),
:deep(.md-preview) {
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

.priority-option {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.priority-swatch {
  width: 0.75rem;
  height: 0.75rem;
  border-radius: 50%;
  display: inline-block;
}
</style>
