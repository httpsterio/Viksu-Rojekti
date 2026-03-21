<script setup lang="ts">
import { ref, watch } from "vue"
import { useBoard } from "@/composables/useBoard"
import type { BoardConfig } from "@/types"
import Dialog from "primevue/dialog"
import Button from "primevue/button"
import InputText from "primevue/inputtext"
import InputGroup from "primevue/inputgroup"
import InputGroupAddon from "primevue/inputgroupaddon"
import { useConfirm } from "primevue/useconfirm"
import { useToast } from "primevue/usetoast"
import { animations } from "@formkit/drag-and-drop"
import { useDragAndDrop } from "@formkit/drag-and-drop/vue"

const { config, cards, saveBoardConfig, updateCard } = useBoard()
const confirm = useConfirm()
const toast = useToast()

const visible = ref(false)
const localConfig = ref<BoardConfig | null>(null)

// Phase 1: DnD setup for Statuses, Epics, and Tags
const [statusesParent, statusValues] = useDragAndDrop<BoardConfig["statuses"][number]>([], {
  nativeDrag: true,
  dragHandle: ".drag-handle",
  plugins: [animations()],
  onSort: (event: any) => {
    if (localConfig.value) localConfig.value.statuses = [...event.values]
  },
})

const [epicsParent, epicValues] = useDragAndDrop<BoardConfig["epics"][number]>([], {
  nativeDrag: true,
  dragHandle: ".drag-handle",
  plugins: [animations()],
  onSort: (event: any) => {
    if (localConfig.value) localConfig.value.epics = [...event.values]
  },
})

const [tagsParent, tagValues] = useDragAndDrop<BoardConfig["tags"][number]>([], {
  nativeDrag: true,
  dragHandle: ".drag-handle",
  plugins: [animations()],
  onSort: (event: any) => {
    if (localConfig.value) localConfig.value.tags = [...event.values]
  },
})

watch(visible, (isVisible) => {
  if (isVisible && localConfig.value) {
    statusValues.value = [...localConfig.value.statuses]
    epicValues.value = [...localConfig.value.epics]
    tagValues.value = [...localConfig.value.tags]
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
    const epicNames = localConfig.value.epics.map((e) => e.name.trim())
    if (new Set(epicNames).size !== epicNames.length) {
      toast.add({ severity: "error", summary: "Validation Error", detail: "Epic names must be unique.", life: 4000 })
      return
    }
    const tagNames = localConfig.value.tags.map((t) => t.name.trim())
    if (new Set(tagNames).size !== tagNames.length) {
      toast.add({ severity: "error", summary: "Validation Error", detail: "Tag names must be unique.", life: 4000 })
      return
    }

    await saveBoardConfig(localConfig.value)
    visible.value = false
  }
}

const addStatus = () => {
  const newStatus = { id: `status-${Date.now()}`, name: "New Status" }
  statusValues.value.push(newStatus)
  if (localConfig.value) localConfig.value.statuses = [...statusValues.value]
}

const removeStatus = (index: number) => {
  if (!localConfig.value) return
  const statusToRemove = statusValues.value[index]
  const cardsInStatus = cards.value.filter((c) => c.status === statusToRemove.id)

  if (cardsInStatus.length === 0) {
    confirm.require({
      message: `Are you sure you want to delete the "${statusToRemove.name}" status?`,
      header: "Delete Status",
      icon: "pi pi-exclamation-triangle",
      acceptClass: "p-button-danger",
      accept: () => {
        statusValues.value.splice(index, 1)
        if (localConfig.value) localConfig.value.statuses = [...statusValues.value]
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
        if (localConfig.value) localConfig.value.statuses = [...statusValues.value]
        if (localConfig.value && localConfig.value.statuses.length > 0) {
          const firstStatusId = localConfig.value.statuses[0].id
          for (const card of cardsInStatus) {
            await updateCard({ ...card, status: firstStatusId })
          }
        }
      },
    })
  }
}

const addEpic = () => {
  const newEpic = { name: "New Epic", color: "#3b82f6" }
  epicValues.value.push(newEpic)
  if (localConfig.value) localConfig.value.epics = [...epicValues.value]
}

const removeEpic = (index: number) => {
  confirm.require({
    message: "Are you sure you want to delete this epic?",
    header: "Delete Epic",
    icon: "pi pi-exclamation-triangle",
    acceptClass: "p-button-danger",
    accept: () => {
      epicValues.value.splice(index, 1)
      if (localConfig.value) localConfig.value.epics = [...epicValues.value]
    },
  })
}

const addTag = () => {
  const newTag = { name: "New Tag", color: "#10b981" }
  tagValues.value.push(newTag)
  if (localConfig.value) localConfig.value.tags = [...tagValues.value]
}

const removeTag = (index: number) => {
  confirm.require({
    message: "Are you sure you want to delete this tag?",
    header: "Delete Tag",
    icon: "pi pi-exclamation-triangle",
    acceptClass: "p-button-danger",
    accept: () => {
      tagValues.value.splice(index, 1)
      if (localConfig.value) localConfig.value.tags = [...tagValues.value]
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
          <div class="section-header">
            <label>Status</label>
          </div>
          <div ref="statusesParent" class="list-editor">
            <div v-for="status in statusValues" :key="status.id" class="list-item">
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
                    @click="removeStatus(statusValues.indexOf(status))"
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
          <div ref="epicsParent" class="list-editor">
            <div v-for="epic in epicValues" :key="epic.name" class="list-item">
              <InputGroup>
                <InputGroupAddon class="drag-handle">
                  <i class="pi pi-bars"></i>
                </InputGroupAddon>
                <InputGroupAddon class="color-addon">
                  <input v-model="epic.color" type="color" class="color-swatch" />
                </InputGroupAddon>
                <InputText v-model="epic.name" placeholder="Epic name" />
                <InputGroupAddon>
                  <Button
                    icon="pi pi-times"
                    text
                    severity="secondary"
                    @click="removeEpic(epicValues.indexOf(epic))"
                  />
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
          <div ref="tagsParent" class="list-editor">
            <div v-for="tag in tagValues" :key="tag.name" class="list-item">
              <InputGroup>
                <InputGroupAddon class="drag-handle">
                  <i class="pi pi-bars"></i>
                </InputGroupAddon>
                <InputGroupAddon class="color-addon">
                  <input v-model="tag.color" type="color" class="color-swatch" />
                </InputGroupAddon>
                <InputText v-model="tag.name" placeholder="Tag name" />
                <InputGroupAddon>
                  <Button
                    icon="pi pi-times"
                    text
                    severity="secondary"
                    @click="removeTag(tagValues.indexOf(tag))"
                  />
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
