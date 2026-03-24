// TODO: replace with actual preset imports once aura-base.ts is populated from GitHub (primeuix/themes tag 4.5.4)
import sane from './sane'
import saneDark from './sane-dark'
import whimsical from './whimsical'
import whimsicalDark from './whimsical-dark'

export const themes: Record<string, { preset: object, isDark: boolean }> = {
  'sane':           { preset: sane,          isDark: false },
  'sane-dark':      { preset: saneDark,      isDark: true  },
  'whimsical':      { preset: whimsical,     isDark: false },
  'whimsical-dark': { preset: whimsicalDark, isDark: true  },
}

export function getPreset(name: string): object {
  return themes[name]?.preset ?? themes['sane'].preset
}
