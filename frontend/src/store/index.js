import Vue from 'vue'
import Vuex from 'vuex'

import Notifications from './modules/notifications'
import Banos from './modules/banos'
import Dashboard from './modules/dashboard'

Vue.use(Vuex)

const debug = process.env.NODE_ENV !== 'production'

export default new Vuex.Store({
  modules: {
    notifications: Notifications,
    banos: Banos,
    dashboard: Dashboard
  },
  strict: debug,
  plugins: debug ? [] : []
})
