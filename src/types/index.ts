export interface BoardConfig {
  name: string
  prefix: string
  nextId: number
  statuses: Status[]
  epics: Epic[]
  tags: Tag[]
  priorities: Priority[]
  doneStatuses: string[]
  hiddenStatuses: string[]
  hiddenStatusesEnabled: boolean
  dueDateThreshold: number
}

export interface Status {
  id?: string
  name: string
  pendingRename?: string // read-only, set by backend during rename
}

export interface Priority {
  name: string
  color: string
}

export interface Epic {
  id?: string
  name: string
  color: string
  pendingRename?: string
}

export interface Tag {
  id?: string
  name: string
  color: string
  pendingRename?: string
}

export interface ChecklistItem {
  id: string
  text: string
  done: boolean
}

export interface CardMeta {
  id: string
  title: string
  status: string
  epic: string | null
  tags: string[]
  priority: number
  position: number
  created: string
  checklist: ChecklistItem[]
  dueDate?: string
}

export interface Card extends CardMeta {
  body: string
}

export interface AllCardsResult {
  cards: Card[]
  errors: string[]
}

export interface ActiveFilters {
  epic: string | null
  tag: string | null
  priority: number | null
}

export interface BoardState {
  theme: string
  collapsedStatuses: string[]
  activeFilters: ActiveFilters
  view: string
  showHiddenLanes: boolean
}

export interface Index {
  generated: string
  cardCount: number
  cards: CardMeta[]
}
