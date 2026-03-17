import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { BoardConfig, Card, Epic } from '@/types'
import { useToast } from 'primevue/usetoast'

const config = ref<BoardConfig | null>(null)
const cards = ref<Card[]>([])
const collapsedLanes = ref<Set<string>>(new Set())
const activeFilters = ref({
  epic: null as string | null,
  tag: null as string | null,
  priority: null as string | null,
  search: ''
})
const currentView = ref<'board' | 'epics'>('board')
const editingCard = ref<Card | null>(null)
const isCreating = ref(false)
const isDarkMode = ref(false)
const isLoading = ref(true)
const needsInit = ref(false)

export function useBoard() {
  const toast = useToast()

  const loadBoard = async () => {
    isLoading.value = true
    try {
      config.value = await invoke<BoardConfig>('get_board_config')
      cards.value = await invoke<Card[]>('get_all_cards')
      needsInit.value = false
    } catch (e) {
      console.error('Failed to load board:', e)
      if (typeof e === 'string' && (e.includes('config file') || e.includes('board.yaml'))) {
        needsInit.value = true
      } else {
        toast.add({ severity: 'error', summary: 'Error', detail: String(e), life: 3000 })
      }
    } finally {
      isLoading.value = false
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
      await invoke('save_board_config', { config: newConfig })
      await loadBoard()
      toast.add({ severity: 'success', summary: 'Success', detail: 'Settings saved', life: 3000 })
    } catch (e) {
      toast.add({ severity: 'error', summary: 'Error', detail: String(e), life: 3000 })
      await loadBoard()
    }
  }

  const createCard = async (cardData: any) => {
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

  const toggleLaneCollapse = (lane: string) => {
    if (collapsedLanes.value.has(lane)) {
      collapsedLanes.value.delete(lane)
    } else {
      collapsedLanes.value.add(lane)
    }
  }

  const toggleDarkMode = () => {
    isDarkMode.value = !isDarkMode.value
    if (isDarkMode.value) {
      document.documentElement.classList.add('dark-mode')
    } else {
      document.documentElement.classList.remove('dark-mode')
    }
  }

  const filteredCards = computed(() => {
    return cards.value.filter(c => {
      if (activeFilters.value.epic && c.epic !== activeFilters.value.epic) return false
      if (activeFilters.value.tag && !c.tags.includes(activeFilters.value.tag)) return false
      if (activeFilters.value.priority && c.priority !== activeFilters.value.priority) return false
      if (activeFilters.value.search && !c.title.toLowerCase().includes(activeFilters.value.search.toLowerCase())) return false
      return true
    })
  })

  const cardsByLane = computed(() => {
    const grouped: Record<string, Card[]> = {}
    if (!config.value) return grouped
    
    for (const lane of config.value.lanes) {
      grouped[lane] = filteredCards.value
        .filter(c => c.status === lane)
        .sort((a, b) => a.position - b.position)
    }
    return grouped
  })

  const cardsByEpic = computed(() => {
    const grouped: Record<string, Card[]> = { unassigned: [] }
    if (!config.value) return grouped

    for (const epic of config.value.epics) {
      grouped[epic.id] = []
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
    collapsedLanes,
    activeFilters,
    currentView,
    editingCard,
    isCreating,
    isDarkMode,
    isLoading,
    needsInit,
    loadBoard,
    initBoard,
    saveBoardConfig,
    createCard,
    updateCard,
    deleteCard,
    moveCard,
    toggleLaneCollapse,
    toggleDarkMode,
    cardsByLane,
    cardsByEpic
  }
}
