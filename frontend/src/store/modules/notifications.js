const state = {
  notifications: []
}

const getters = {
  notifications: state => state.notifications
}

const mutations = {
  addNotification: (state, notification) => {
    if (!notification.key) {
      notification = { ...notification, key: Date.now() }
    }
    state.notifications.unshift(notification)
  },
  removeNotification: (state, key) => {
    const i = state.notifications.findIndex(obj => obj.key === key)
    console.log('found notification ' + i)
    if (i >= 0) {
      state.notifications.splice(i, 1)
    }
  }
}

const actions = {
  addNotification: ({ commit }, { title, message, theme, timeout }) => {
    // We add a key, which is required in a <transition-group> (this is how Notifications are displayed)
    const notification = { title, message, theme, timeout, key: Date.now() }
    commit('addNotification', notification)
    setTimeout(() => commit('removeNotification', notification.key), notification.timeout)
  },
  removeNotification: ({ commit }, key) => {
    console.log('action: remove ' + key)
    commit('removeNotification', key)
  }
}

export default {
  namespaced: true,
  state,
  getters,
  mutations,
  actions
}
