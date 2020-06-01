import Vue from 'vue'
import App from '@/App'
import { AppRouter } from '@/router'
import store from './store'
import Truncate from './mixins/truncate'

import configureAxios from '@/api/axiosConfig'

import '@/assets/styles/index.css'

Vue.config.productionTip = false
Vue.mixin(Truncate)

configureAxios()

/* eslint-disable no-new */
new Vue({
  el: '#app',
  store: store,
  router: AppRouter,
  components: { App },
  render: h => h(App)
})
