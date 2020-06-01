const state = {
  panel: 'Default',
  key: '',
  value: ''
}

const getters = {
  panel: state => state.panel,
  key: state => state.key,
  value: state => state.value
}

const mutations = {
  switchPanel: (state, { panel, key, value }) => {
    state.panel = panel
    if (typeof key !== 'undefined') {
      state.key = key
    }
    if (typeof value !== 'undefined') {
      state.value = value
    }
  }
}

const actions = {
  switchPanel: ({ commit }, { panel, key, value }) => {
    commit('switchPanel', { panel, key, value })
  }
}

export default {
  namespaced: true,
  state,
  getters,
  mutations,
  actions
}
