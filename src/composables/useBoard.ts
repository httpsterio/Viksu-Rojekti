import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { BoardConfig, Ticket, Epic } from '@/types'
import { useToast } from 'primevue/usetoast'

const config = ref<BoardConfig | null>(null)
const tickets = ref<Ticket[]>([])
const collapsedLanes = ref<Set<string>>(new Set())
const activeFilters = ref({
  epic: null as string | null,
  tag: null as string | null,
  priority: null as string | null,
  search: ''
})
const currentView = ref<'board' | 'epics'>('board')
const editingTicket = ref<Ticket | null>(null)
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
      tickets.value = await invoke<Ticket[]>('get_all_tickets')
      needsInit.value = false
    } catch (e) {
      console.error('Failed to load board:', e)
      if (typeof e === 'string' && e.includes('board.yaml')) {
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

  const createTicket = async (ticketData: any) => {
    try {
      const newTicket = await invoke<Ticket>('create_ticket', ticketData)
      tickets.value.push(newTicket)
      toast.add({ severity: 'success', summary: 'Success', detail: 'Ticket created', life: 3000 })
      return newTicket
    } catch (e) {
      toast.add({ severity: 'error', summary: 'Error', detail: String(e), life: 3000 })
    }
  }

  const updateTicket = async (ticket: Ticket) => {
    try {
      const updatedTicket = await invoke<Ticket>('update_ticket', { ticket })
      const index = tickets.value.findIndex(t => t.id === updatedTicket.id)
      if (index !== -1) {
        tickets.value[index] = updatedTicket
      }
      toast.add({ severity: 'success', summary: 'Success', detail: 'Ticket updated', life: 3000 })
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

  const deleteTicket = async (id: string) => {
    try {
      await invoke('delete_ticket', { id })
      tickets.value = tickets.value.filter(t => t.id !== id)
      toast.add({ severity: 'success', summary: 'Success', detail: 'Ticket deleted', life: 3000 })
    } catch (e) {
      toast.add({ severity: 'error', summary: 'Error', detail: String(e), life: 3000 })
    }
  }

  const moveTicket = async (id: string, newStatus: string, newPosition: number) => {
    try {
      const updatedTicket = await invoke<Ticket>('move_ticket', { id, newStatus, newPosition })
      const index = tickets.value.findIndex(t => t.id === id)
      if (index !== -1) {
        tickets.value[index] = updatedTicket
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

  const filteredTickets = computed(() => {
    return tickets.value.filter(t => {
      if (activeFilters.value.epic && t.epic !== activeFilters.value.epic) return false
      if (activeFilters.value.tag && !t.tags.includes(activeFilters.value.tag)) return false
      if (activeFilters.value.priority && t.priority !== activeFilters.value.priority) return false
      if (activeFilters.value.search && !t.title.toLowerCase().includes(activeFilters.value.search.toLowerCase())) return false
      return true
    })
  })

  const ticketsByLane = computed(() => {
    const grouped: Record<string, Ticket[]> = {}
    if (!config.value) return grouped
    
    for (const lane of config.value.lanes) {
      grouped[lane] = filteredTickets.value
        .filter(t => t.status === lane)
        .sort((a, b) => a.position - b.position)
    }
    return grouped
  })

  const ticketsByEpic = computed(() => {
    const grouped: Record<string, Ticket[]> = { unassigned: [] }
    if (!config.value) return grouped

    for (const epic of config.value.epics) {
      grouped[epic.id] = []
    }

    for (const ticket of filteredTickets.value) {
      const key = ticket.epic || 'unassigned'
      if (grouped[key]) {
        grouped[key].push(ticket)
      } else {
        grouped.unassigned.push(ticket)
      }
    }
    return grouped
  })

  return {
    config,
    tickets,
    collapsedLanes,
    activeFilters,
    currentView,
    editingTicket,
    isCreating,
    isDarkMode,
    isLoading,
    needsInit,
    loadBoard,
    initBoard,
    createTicket,
    updateTicket,
    deleteTicket,
    moveTicket,
    toggleLaneCollapse,
    toggleDarkMode,
    ticketsByLane,
    ticketsByEpic
  }
}
