const state = {
  counter: 0
}

const getters = {
  counter: state => state.counter
}

const mutations = {
  updateCounter: (state, value) => { state.counter = value }
}

const actions = {
  updateCounter: async ({ dispatch, commit }, { value }) => {
    commit('updateCounter', { value })
  }
}

export default {
  namespaced: true,
  state,
  getters,
  actions,
  mutations
}
