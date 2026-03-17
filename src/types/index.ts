export interface BoardConfig {
  name: string
  prefix: string
  nextId: number
  lanes: string[]
  epics: Epic[]
  tags: string[]
  priorities: string[]
}

export interface Epic {
  id: string
  name: string
  color: string
}

export interface TicketMeta {
  id: string
  title: string
  status: string
  epic: string | null
  tags: string[]
  priority: string
  position: number
  created: string
}

export interface Ticket extends TicketMeta {
  body: string
}

export interface Index {
  generated: string
  ticketCount: number
  tickets: TicketMeta[]
}
