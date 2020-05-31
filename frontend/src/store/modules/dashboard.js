const state = {
  panel: 'Default'
}

const getters = {
  panel: state => state.panel
}

const mutations = {
  switchPanel: (state, panel) => {
    state.panel = panel
    console.log(state.panel)
  }
}

const actions = {
  switchPanel: ({ commit }, panel) => {
    commit('switchPanel', panel)
  }
}

export default {
  namespaced: true,
  state,
  getters,
  mutations,
  actions
}
