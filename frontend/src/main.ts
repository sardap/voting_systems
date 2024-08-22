import { createApp } from 'vue'
import { createPinia } from 'pinia'
import VNetworkGraph from 'v-network-graph'
import 'v-network-graph/lib/style.css'

import App from './App.vue'
import router from './router'
import Vue3EasyDataTable from 'vue3-easy-data-table'

import './assets/main.css'
import 'vue3-easy-data-table/dist/style.css'

const app = createApp(App)

app.component('EasyDataTable', Vue3EasyDataTable)

app.use(createPinia())
app.use(router)
app.use(VNetworkGraph)

app.mount('#app')
