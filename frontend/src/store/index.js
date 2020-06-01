import Vue from 'vue'
import Vuex from 'vuex'

import Notifications from './modules/notifications'
import Banos from './modules/banos'
import Dashboard from './modules/dashboard'
import Features from './modules/features'
import Scenarios from './modules/scenarios'
import Steps from './modules/steps'

Vue.use(Vuex)

const debug = process.env.NODE_ENV !== 'production'

export default new Vuex.Store({
  modules: {
    notifications: Notifications,
    banos: Banos,
    dashboard: Dashboard,
    features: Features,
    scenarios: Scenarios,
    steps: Steps
  },
  strict: debug,
  plugins: debug ? [] : []
})
