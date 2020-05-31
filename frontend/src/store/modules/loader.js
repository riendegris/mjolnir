// dnlytras

const state = {
  loading: false
}
const getters = {
  loading: state => state.loading,
  loadingStatus: state => state.loading ? 'Fetching stuff' : 'Ready'
}
const mutations = {
  updateLoader: (state, status) => { state.loading = status }
}
const actions = {
  executeWithLoader: async ({ commit, dispatch }, fn) => {
    commit('updateLoader', true)
    await dispatch(fn, { root: true })
    commit('updateLoader', false)
  }
}

export default {
  state, getters, mutations, actions
}
