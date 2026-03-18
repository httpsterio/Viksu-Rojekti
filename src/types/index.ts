export interface BoardConfig {
  name: string
  prefix: string
  nextId: number
  lanes: string[]
  epics: Epic[]
  tags: Tag[]
  priorities: string[]
}

export interface Epic {
  id: string
  name: string
  color: string
}

export interface Tag {
  id: string
  name: string
  color: string
}

export interface CardMeta {
  id: string
  title: string
  status: string
  epic: string | null
  tags: string[]
  priority: string
  position: number
  created: string
}

export interface Card extends CardMeta {
  body: string
}

export interface Index {
  generated: string
  cardCount: number
  cards: CardMeta[]
}
