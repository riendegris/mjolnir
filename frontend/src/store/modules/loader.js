const state = {
  loading: false
}
const mutations = {
  show: (state) => { state.loading = true },
  hide: (state) => { state.loading = false }
}
const actions = {
  show ({ commit }) {
    commit('show')
  },
  hide ({ commit }) {
    commit('hide')
  }
}

export default {
  namespaced: true,
  state,
  mutations,
  actions
}
