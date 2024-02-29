import { Suspense, createApp, defineComponent, h } from 'vue'
import { createPinia } from 'pinia'

import App from './App.vue'
import router from './router'

import './style.css'

const app = createApp(
  defineComponent({
    render() {
      return h(Suspense, null, { default: () => h(App) })
    },
  })
)

app.use(createPinia())
app.use(router)

app.mount('#app')
