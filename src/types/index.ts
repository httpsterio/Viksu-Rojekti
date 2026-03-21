export interface BoardConfig {
  name: string
  prefix: string
  nextId: number
  statuses: Status[]
  epics: Epic[]
  tags: Tag[]
  priorities: Priority[]
}

export interface Status {
  id: string
  name: string
}

export interface Priority {
  name: string
  color: string
}

export interface Epic {
  name: string
  color: string
  pendingRename?: string
}

export interface Tag {
  name: string
  color: string
  pendingRename?: string
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
}

export interface Card extends CardMeta {
  body: string
}

export interface AllCardsResult {
  cards: Card[]
  errors: string[]
}

export interface Index {
  generated: string
  cardCount: number
  cards: CardMeta[]
}
