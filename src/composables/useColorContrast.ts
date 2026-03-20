export function useColorContrast() {
  function contrastColor(hex: string | undefined | null): string {
    if (!hex || hex.length < 7) return "#000000"

    // Parse hex to RGB
    const r = parseInt(hex.slice(1, 3), 16)
    const g = parseInt(hex.slice(3, 5), 16)
    const b = parseInt(hex.slice(5, 7), 16)

    // Perceptual luminance calculation (ITU-R BT.601)
    const luminance = (0.299 * r + 0.587 * g + 0.114 * b) / 255

    return luminance > 0.5 ? "#000000" : "#ffffff"
  }

  return { contrastColor }
}
