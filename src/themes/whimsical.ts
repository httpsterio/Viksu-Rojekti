import auraBase from "./aura-base"

export default {
  ...auraBase,
  semantic: {
    ...auraBase.semantic,
    primary: {
      ...auraBase.semantic.primary,
      500: "#ff0000",
    },
  },
}
