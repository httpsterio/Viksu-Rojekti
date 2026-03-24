import { ref, computed, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { BoardConfig, BoardState, Card, AllCardsResult } from '@/types'
import { useToast } from 'primevue/usetoast'
import { themes, applyTheme, nextTheme, THEME_CYCLE, type ThemeName } from '@/themes'

const config = ref<BoardConfig | null>(null)
const cards = ref<Card[]>([])
const collapsedStatuses = ref<Set<string>>(new Set())
const activeFilters = ref({
  epic: null as string | null,
  tag: null as string | null,
  priority: null as number | null,
  search: ''
})
const currentView = ref<'board' | 'epics'>('board')
const editingCard = ref<Card | null>(null)
const isCreating = ref(false)
const currentTheme = ref<ThemeName>('sane')
const isDark = computed(() => themes[currentTheme.value]?.isDark ?? false)
const themeIcon = computed(() => themes[currentTheme.value]?.icon ?? 'pi pi-chevron-up')
const isLoading = ref(true)
const needsInit = ref(false)
const draggedCardId = ref<string | null>(null)
let loadGeneration = 0
let stateLoaded = false
let watchersInitialized = false

export function useBoard() {
  const toast = useToast()
  const saveState = async () => {
    if (!stateLoaded) return
    try {
      await invoke('save_board_state', {
        boardState: {
          theme: currentTheme.value,
          collapsedStatuses: [...collapsedStatuses.value],
          activeFilters: {
            epic: activeFilters.value.epic,
            tag: activeFilters.value.tag,
            priority: activeFilters.value.priority,
          },
          view: currentView.value,
        } satisfies BoardState,
      })
    } catch (e) {
      console.error('Failed to save board state:', e)
    }
  }

  const loadBoard = async (silent = false) => {
    const generation = ++loadGeneration
    if (!silent) isLoading.value = true
    try {
      const [newConfig, result, boardState] = await Promise.all([
        invoke<BoardConfig>('get_board_config'),
        invoke<AllCardsResult>('get_all_cards'),
        invoke<BoardState>('get_board_state'),
      ])
      if (generation === loadGeneration) {
        config.value = newConfig
        cards.value = result.cards

        if (!stateLoaded) {
          const savedTheme = THEME_CYCLE.includes(boardState.theme as ThemeName) ? boardState.theme as ThemeName : 'sane'
          currentTheme.value = savedTheme
          applyTheme(savedTheme)
          collapsedStatuses.value = new Set(boardState.collapsedStatuses)
          activeFilters.value.epic = boardState.activeFilters.epic
          activeFilters.value.tag = boardState.activeFilters.tag
          activeFilters.value.priority = boardState.activeFilters.priority
          currentView.value = boardState.view as 'board' | 'epics'
          stateLoaded = true

          if (!watchersInitialized) {
            watchersInitialized = true
            watch(() => currentTheme.value, saveState)
            watch(() => currentView.value, saveState)
            watch(() => [...collapsedStatuses.value], saveState, { deep: true })
            watch(
              () => [activeFilters.value.epic, activeFilters.value.tag, activeFilters.value.priority],
              saveState
            )
          }
        }

        for (const error of result.errors) {
          toast.add({ severity: 'warn', summary: 'Could not load card', detail: error, life: 6000 })
        }

        needsInit.value = false
      }
    } catch (e) {
      if (generation !== loadGeneration) return
      console.error('Failed to load board:', e)
      if (typeof e === 'string' && (e.includes('config file') || e.includes('board.yaml'))) {
        needsInit.value = true
      } else {
        toast.add({ severity: 'error', summary: 'Error', detail: String(e), life: 3000 })
      }
    } finally {
      if (!silent && generation === loadGeneration) {
        isLoading.value = false
      }
    }
  }
  const initBoard = async (name: string, prefix: string) => {
    try {
      await invoke('init_project', { name, prefix })
      await loadBoard()
      toast.add({ severity: 'success', summary: 'Success', detail: 'Board initialized', life: 3000 })
    } catch (e) {
      toast.add({ severity: 'error', summary: 'Error', detail: String(e), life: 3000 })
    }
  }

  const saveBoardConfig = async (newConfig: BoardConfig) => {
    try {
      config.value = newConfig

      // Clear stale filters
      if (activeFilters.value.epic && !newConfig.epics.some(e => e.name === activeFilters.value.epic)) {
        activeFilters.value.epic = null
      }
      if (activeFilters.value.tag && !newConfig.tags.some(t => t.name === activeFilters.value.tag)) {
        activeFilters.value.tag = null
      }
      if (activeFilters.value.priority && activeFilters.value.priority > newConfig.priorities.length) {
        activeFilters.value.priority = null
      }

      await invoke('save_board_config', { config: newConfig })
      await loadBoard()
      toast.add({ severity: 'success', summary: 'Success', detail: 'Settings saved', life: 3000 })
    } catch (e) {
      toast.add({ severity: 'error', summary: 'Error', detail: String(e), life: 3000 })
      await loadBoard()
    }
  }

  const createCard = async (cardData: Omit<Card, 'id' | 'created' | 'position'>) => {
    try {
      const newCard = await invoke<Card>('create_card', cardData)
      if (!cards.value.some(c => c.id === newCard.id)) {
        cards.value.push(newCard)
      }
      toast.add({ severity: 'success', summary: 'Success', detail: 'Card created', life: 3000 })
      return newCard
    } catch (e) {
      toast.add({ severity: 'error', summary: 'Error', detail: String(e), life: 3000 })
    }
  }

  const updateCard = async (card: Card) => {
    try {
      const updatedCard = await invoke<Card>('update_card', { card })
      const index = cards.value.findIndex(c => c.id === updatedCard.id)
      if (index !== -1) {
        cards.value[index] = updatedCard
      }
      toast.add({ severity: 'success', summary: 'Success', detail: 'Card updated', life: 3000 })
    } catch (e) {
      toast.add({ severity: 'error', summary: 'Error', detail: String(e), life: 3000 })
    }
  }

  const deleteCard = async (id: string) => {
    try {
      await invoke('delete_card', { id })
      cards.value = cards.value.filter(c => c.id !== id)
      toast.add({ severity: 'success', summary: 'Success', detail: 'Card deleted', life: 3000 })
    } catch (e) {
      toast.add({ severity: 'error', summary: 'Error', detail: String(e), life: 3000 })
    }
  }

  const moveCard = async (id: string, newStatus: string, newPosition: number) => {
    try {
      const updatedCard = await invoke<Card>('move_card', { id, newStatus, newPosition })
      const index = cards.value.findIndex(c => c.id === id)
      if (index !== -1) {
        cards.value[index] = updatedCard
      }
    } catch (e) {
      toast.add({ severity: 'error', summary: 'Error', detail: String(e), life: 3000 })
    }
  }

  const toggleStatusCollapse = (statusId: string) => {
    if (collapsedStatuses.value.has(statusId)) {
      collapsedStatuses.value.delete(statusId)
    } else {
      collapsedStatuses.value.add(statusId)
    }
  }

  const cycleTheme = () => {
    const next = nextTheme(currentTheme.value)
    currentTheme.value = next
    applyTheme(next)
  }

  const filteredCards = computed(() => {
    return cards.value.filter(c => {
      if (activeFilters.value.epic && c.epic !== activeFilters.value.epic) return false
      if (activeFilters.value.tag && !c.tags?.includes(activeFilters.value.tag)) return false
      if (activeFilters.value.priority && c.priority !== activeFilters.value.priority) return false
      if (activeFilters.value.search) {
        const query = activeFilters.value.search.toLowerCase()
        const matchesTitle = c.title.toLowerCase().includes(query)
        const matchesId = c.id.toLowerCase().includes(query)
        if (!matchesTitle && !matchesId) return false
      }
      return true
    })
  })

  const cardsByStatus = computed(() => {
    const grouped: Record<string, Card[]> = {}
    if (!config.value) return grouped
    
    for (const status of config.value.statuses) {
      grouped[status.name] = filteredCards.value
        .filter(c => c.status === status.name)
        .sort((a, b) => a.position - b.position)
    }
    return grouped
  })

  const cardsByEpic = computed(() => {
    const grouped: Record<string, Card[]> = { unassigned: [] }
    if (!config.value) return grouped

    for (const epic of config.value.epics) {
      grouped[epic.name] = []
    }

    for (const card of filteredCards.value) {
      const key = card.epic || 'unassigned'
      if (grouped[key]) {
        grouped[key].push(card)
      } else {
        grouped.unassigned.push(card)
      }
    }
    return grouped
  })

  return {
    config,
    cards,
    collapsedStatuses,
    activeFilters,
    currentView,
    editingCard,
    isCreating,
    currentTheme,
    isDark,
    themeIcon,
    draggedCardId,
    isLoading,
    needsInit,
    loadBoard,
    initBoard,
    saveBoardConfig,
    createCard,
    updateCard,
    deleteCard,
    moveCard,
    toggleStatusCollapse,
    cycleTheme,
    cardsByStatus,
    cardsByEpic
  }
}
