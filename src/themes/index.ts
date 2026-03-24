import { updatePreset } from "@primevue/themes"
import sane from "./sane"
import saneDark from "./sane-dark"
import whimsical from "./whimsical"
import whimsicalDark from "./whimsical-dark"

export const THEME_CYCLE = ["sane", "sane-dark", "whimsical", "whimsical-dark"] as const
export type ThemeName = (typeof THEME_CYCLE)[number]

export const themes: Record<ThemeName, { preset: Record<string, unknown>; isDark: boolean; icon: string }> = {
  sane: { preset: sane, isDark: false, icon: "pi pi-chevron-left" },
  "sane-dark": { preset: saneDark, isDark: true, icon: "pi pi-chevron-up" },
  whimsical: { preset: whimsical, isDark: false, icon: "pi pi-chevron-right" },
  "whimsical-dark": { preset: whimsicalDark, isDark: true, icon: "pi pi-chevron-down" },
}

export function getPreset(name: string): Record<string, unknown> {
  return themes[name as ThemeName]?.preset ?? themes["sane"].preset
}

export function applyTheme(name: string) {
  const preset = getPreset(name)
  updatePreset(preset)
  document.documentElement.setAttribute("data-theme", name)
}

export function nextTheme(current: string): ThemeName {
  const idx = THEME_CYCLE.indexOf(current as ThemeName)
  return THEME_CYCLE[(idx + 1) % THEME_CYCLE.length]
}
