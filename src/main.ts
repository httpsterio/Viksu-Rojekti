import { createApp } from 'vue'
import PrimeVue from 'primevue/config'
import Aura from '@primevue/themes/aura'
// import Material from '@primevue/themes/material'
// import Lara from '@primevue/themes/lara'
// import Nora from '@primevue/themes/nora'
import ToastService from 'primevue/toastservice'
import ConfirmationService from 'primevue/confirmationservice'
import 'primeicons/primeicons.css'
import './styles/main.css'
import App from './App.vue'

const app = createApp(App)

app.use(PrimeVue, {
  theme: {
    preset: Aura,
    options: {
      darkModeSelector: '.dark-mode'
    }
  }
})

app.use(ToastService)
app.use(ConfirmationService)

app.mount('#app')
