import Vue from 'vue'
import App from '@/App'
import { AppRouter } from '@/router'
import store from './store'

import configureAxios from '@/api/axiosConfig'

import '@/assets/styles/index.css'

Vue.config.productionTip = false

configureAxios()

Vue.config.productionTip = false

/* eslint-disable no-new */
new Vue({
  el: '#app',
  store: store,
  router: AppRouter,
  components: { App },
  render: h => h(App)
})
