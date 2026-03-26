<script setup lang="ts">
import { ref, watch, nextTick } from "vue"
import { useBoard } from "@/composables/useBoard"
import type { BoardConfig } from "@/types"

type WithDragId<T> = T & { _dragId: string }
import Dialog from "primevue/dialog"
import Button from "primevue/button"
import InputText from "primevue/inputtext"
import InputGroup from "primevue/inputgroup"
import InputGroupAddon from "primevue/inputgroupaddon"
import MultiSelect from "primevue/multiselect"
import { useConfirm } from "primevue/useconfirm"
import { useToast } from "primevue/usetoast"
import { animations, tearDown } from "@formkit/drag-and-drop"
import { dragAndDrop } from "@formkit/drag-and-drop/vue"

const { config, cards, saveBoardConfig, updateCard } = useBoard()
const confirm = useConfirm()
const toast = useToast()

const visible = ref(false)
const localConfig = ref<BoardConfig | null>(null)

const statusesParent = ref<HTMLElement | undefined>()
const statusValues = ref<WithDragId<BoardConfig["statuses"][number]>[]>([])
const epicsParent = ref<HTMLElement | undefined>()
const epicValues = ref<WithDragId<BoardConfig["epics"][number]>[]>([])
const tagsParent = ref<HTMLElement | undefined>()
const tagValues = ref<WithDragId<BoardConfig["tags"][number]>[]>([])

watch(visible, async (isVisible) => {
  if (!localConfig.value) return
  if (isVisible) {
    statusValues.value = localConfig.value.statuses.map((s, i) => ({ ...s, _dragId: `s${i}` }))
    epicValues.value = localConfig.value.epics.map((e, i) => ({ ...e, _dragId: `e${i}` }))
    tagValues.value = localConfig.value.tags.map((t, i) => ({ ...t, _dragId: `t${i}` }))
    await nextTick()
    const dndConfig = {
      dragHandle: ".drag-handle",
      nativeDrag: true,
      draggingClass: "dragging-setting",
      dragPlaceholderClass: "ghost-setting",
      plugins: [animations()],
    }
    if (statusesParent.value)
      dragAndDrop({ parent: statusesParent.value, values: statusValues, ...dndConfig })
    if (epicsParent.value)
      dragAndDrop({ parent: epicsParent.value, values: epicValues, ...dndConfig })
    if (tagsParent.value) dragAndDrop({ parent: tagsParent.value, values: tagValues, ...dndConfig })
  } else {
    if (statusesParent.value) tearDown(statusesParent.value)
    if (epicsParent.value) tearDown(epicsParent.value)
    if (tagsParent.value) tearDown(tagsParent.value)
  }
})

const open = () => {
  if (config.value) {
    localConfig.value = JSON.parse(JSON.stringify(config.value))
    visible.value = true
  }
}

defineExpose({ open })

const handleSave = async () => {
  if (localConfig.value) {
    const epicNames = epicValues.value.map((e) => e.name.trim())
    if (new Set(epicNames).size !== epicNames.length) {
      toast.add({
        severity: "error",
        summary: "Validation Error",
        detail: "Epic names must be unique.",
        life: 4000,
      })
      return
    }
    const tagNames = tagValues.value.map((t) => t.name.trim())
    if (new Set(tagNames).size !== tagNames.length) {
      toast.add({
        severity: "error",
        summary: "Validation Error",
        detail: "Tag names must be unique.",
        life: 4000,
      })
      return
    }
    const statusNames = statusValues.value.map((s) => s.name.trim())
    if (new Set(statusNames).size !== statusNames.length) {
      toast.add({
        severity: "error",
        summary: "Validation Error",
        detail: "Status names must be unique.",
        life: 4000,
      })
      return
    }

    localConfig.value.statuses = statusValues.value.map(({ _dragId: _, ...s }) => s)
    localConfig.value.epics = epicValues.value.map(({ _dragId: _, ...e }) => e)
    localConfig.value.tags = tagValues.value.map(({ _dragId: _, ...t }) => t)
    await saveBoardConfig(localConfig.value)
    visible.value = false
  }
}

const addStatus = () => {
  statusValues.value.push({ id: crypto.randomUUID(), name: "New Status", _dragId: `s${Date.now()}` })
}

const removeStatus = (index: number) => {
  if (!localConfig.value) return
  const statusToRemove = statusValues.value[index]
  const cardsInStatus = cards.value.filter((c) => c.status === statusToRemove.name)

  if (cardsInStatus.length === 0) {
    confirm.require({
      message: `Are you sure you want to delete the "${statusToRemove.name}" status?`,
      header: "Delete Status",
      icon: "pi pi-exclamation-triangle",
      acceptClass: "p-button-danger",
      accept: () => {
        statusValues.value.splice(index, 1)
      },
    })
  } else {
    confirm.require({
      message: `The status "${statusToRemove.name}" has ${cardsInStatus.length} cards assigned to it. If you proceed, these cards will be moved to the first status in the list. Do you want to proceed?`,
      header: "Delete Status & Move Cards",
      icon: "pi pi-exclamation-triangle",
      acceptClass: "p-button-danger",
      accept: async () => {
        statusValues.value.splice(index, 1)
        if (statusValues.value.length > 0) {
          const firstStatusName = statusValues.value[0].name
          for (const card of cardsInStatus) {
            await updateCard({ ...card, status: firstStatusName })
          }
        }
      },
    })
  }
}

const addEpic = () => {
  epicValues.value.push({
    id: crypto.randomUUID(),
    name: "New Epic",
    color: "#3b82f6",
    _dragId: `e${Date.now()}`,
  })
}

const removeEpic = (index: number) => {
  confirm.require({
    message: "Are you sure you want to delete this epic?",
    header: "Delete Epic",
    icon: "pi pi-exclamation-triangle",
    acceptClass: "p-button-danger",
    accept: () => {
      epicValues.value.splice(index, 1)
    },
  })
}

const addTag = () => {
  tagValues.value.push({
    id: crypto.randomUUID(),
    name: "New Tag",
    color: "#10b981",
    _dragId: `t${Date.now()}`,
  })
}

const removeTag = (index: number) => {
  confirm.require({
    message: "Are you sure you want to delete this tag?",
    header: "Delete Tag",
    icon: "pi pi-exclamation-triangle",
    acceptClass: "p-button-danger",
    accept: () => {
      tagValues.value.splice(index, 1)
    },
  })
}
</script>

<template>
  <Dialog
    v-model:visible="visible"
    modal
    header="Board Settings"
    class="settings-modal"
    :dismissable-mask="true"
    :draggable="false"
  >
    <div v-if="localConfig" class="settings-layout">
      <div class="settings-column">
        <section>
          <label>Board Name</label>
          <InputText v-model="localConfig.name" fluid />
        </section>

        <section>
          <label>Done Statuses</label>
          <MultiSelect
            v-model="localConfig.doneStatuses"
            :options="statusValues"
            option-label="name"
            option-value="id"
            placeholder="Select Done statuses"
            :max-selected-labels="3"
            class="w-full"
            fluid
          />
          <p class="section-help">Cards in these statuses are considered finished.</p>
        </section>

        <section>
          <div class="section-header">
            <label>Status</label>
          </div>
          <div ref="statusesParent" class="list-editor" @dragover.capture.prevent @dragenter.capture.prevent>
            <div v-for="(status, index) in statusValues" :key="status._dragId" class="list-item">
              <InputGroup>
                <InputGroupAddon class="drag-handle">
                  <i class="pi pi-bars"></i>
                </InputGroupAddon>
                <InputText v-model="status.name" size="small" />
                <InputGroupAddon>
                  <Button
                    icon="pi pi-trash"
                    text
                    severity="danger"
                    size="small"
                    @click="removeStatus(index)"
                  />
                </InputGroupAddon>
              </InputGroup>
            </div>
          </div>
          <Button
            icon="pi pi-plus"
            label="Add Status"
            size="small"
            class="add-btn"
            style="margin-top: 0.5rem"
            @click="addStatus"
          />
        </section>

        <section>
          <div class="section-header">
            <label>Priorities</label>
          </div>
          <div class="list-editor">
            <div v-for="(priority, index) in localConfig.priorities" :key="index" class="list-item">
              <InputGroup>
                <InputGroupAddon>{{ index + 1 }}</InputGroupAddon>
                <InputGroupAddon class="color-addon">
                  <input v-model="priority.color" type="color" class="color-swatch" />
                </InputGroupAddon>
                <InputText
                  v-model="priority.name"
                  size="small"
                  :placeholder="['Critical', 'Severe', 'Substantial', 'Moderate', 'Low'][index]"
                />
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
          <div ref="epicsParent" class="list-editor" @dragover.capture.prevent @dragenter.capture.prevent>
            <div v-for="(epic, index) in epicValues" :key="epic._dragId" class="list-item">
              <InputGroup>
                <InputGroupAddon class="drag-handle">
                  <i class="pi pi-bars"></i>
                </InputGroupAddon>
                <InputGroupAddon class="color-addon">
                  <input v-model="epic.color" type="color" class="color-swatch" />
                </InputGroupAddon>
                <InputText v-model="epic.name" placeholder="Epic name" />
                <InputGroupAddon>
                  <Button icon="pi pi-times" text severity="secondary" @click="removeEpic(index)" />
                </InputGroupAddon>
              </InputGroup>
            </div>
          </div>
          <Button
            icon="pi pi-plus"
            label="Add Epic"
            size="small"
            class="add-btn"
            style="margin-top: 0.5rem"
            @click="addEpic"
          />
        </section>

        <section>
          <div class="section-header">
            <label>Tags</label>
          </div>
          <div ref="tagsParent" class="list-editor" @dragover.capture.prevent @dragenter.capture.prevent>
            <div v-for="(tag, index) in tagValues" :key="tag._dragId" class="list-item">
              <InputGroup>
                <InputGroupAddon class="drag-handle">
                  <i class="pi pi-bars"></i>
                </InputGroupAddon>
                <InputGroupAddon class="color-addon">
                  <input v-model="tag.color" type="color" class="color-swatch" />
                </InputGroupAddon>
                <InputText v-model="tag.name" placeholder="Tag name" />
                <InputGroupAddon>
                  <Button icon="pi pi-times" text severity="secondary" @click="removeTag(index)" />
                </InputGroupAddon>
              </InputGroup>
            </div>
          </div>
          <Button
            icon="pi pi-plus"
            label="Add Tag"
            size="small"
            class="add-btn"
            style="margin-top: 0.5rem"
            @click="addTag"
          />
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

.section-help {
  font-size: 0.75rem;
  color: var(--text-muted);
  margin-top: 0.25rem;
  margin-bottom: 0;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.list-editor {
  display: flex;
  flex-direction: column;
  overflow-y: auto;
  padding-right: 0.5rem;
}

.list-item {
  display: block;
  padding-bottom: 0rem;
  padding-inline: 0.5rem;
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

:deep(.ghost-setting) {
  background: transparent !important;
  border: 2px dashed var(--border-color) !important;
  border-radius: 6px;
  box-shadow: none !important;
}

:deep(.ghost-setting) * {
  visibility: hidden;
}

:deep(.dragging-setting) {
  cursor: grabbing !important;
  user-select: none;
  opacity: 0.8;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
}
</style>
