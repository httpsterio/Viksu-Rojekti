import { createApp } from "vue"
import { invoke } from "@tauri-apps/api/core"
import { config as mdConfig } from "md-editor-v3"
import PrimeVue from "primevue/config"
import ToastService from "primevue/toastservice"
import ConfirmationService from "primevue/confirmationservice"
import "primeicons/primeicons.css"
import "md-editor-v3/lib/style.css"
import "./styles/main.css"
import "./styles/themes.css"
import App from "./App.vue"
import { getPreset, THEME_CYCLE, type ThemeName } from "@/themes"
import type { BoardState } from "@/types"

mdConfig({})

const boardState = await invoke<BoardState>("get_board_state").catch(
  () => ({ theme: "sane" }) as BoardState,
)
const initialTheme: ThemeName = THEME_CYCLE.includes(boardState.theme as ThemeName)
  ? (boardState.theme as ThemeName)
  : "sane"
document.documentElement.setAttribute("data-theme", initialTheme)

const app = createApp(App)

app.use(PrimeVue, {
  theme: {
    preset: getPreset(initialTheme),
    options: {
      darkModeSelector: false,
    },
  },
})

app.use(ToastService)
app.use(ConfirmationService)

app.mount("#app")
