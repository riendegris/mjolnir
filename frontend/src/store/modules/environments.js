import axios from 'axios'
import ApiRoutes from '@/api/apiRoutes'

const state = {
  environments: []
}

const getters = {
  environments: state => state.environments,
  environment: state => (id) => {
    const i = state.environments.findIndex(obj => obj.id === id)
    return state.environments[i]
  },
  indexes: state => (id) => {
    const i = state.environments.findIndex(obj => obj.id === id)
    if (typeof state.environments[i].indexes === 'undefined') {
      return []
    }
    return state.environments[i].indexes
  }
}

const mutations = {
  updateEnvironments: (state, environments) => { state.environments = environments },
  // Update the indexes of the environment identified by 'id'
  updateEnvironmentIndexes: (state, { id, indexes }) => {
    const i = state.environments.findIndex(obj => obj.id === id)
    state.environments[i].indexes = indexes
  }
}

const actions = {
  loadEnvironments: async ({ dispatch, commit }) => {
    const query = `{
      environments {
        id,
        signature,
        status,
        createdAt,
        updatedAt
      }
    }`

    try {
      await axios({
        method: 'post',
        headers: {
          Accept: 'application/json',
          'Content-Type': 'application/json'
        },
        url: ApiRoutes.GraphQL,
        data: JSON.stringify({
          query: query
        })
      }).then(response => {
        if (response.data.errors) {
          const errmsg = response.data.errors[0].message + ': ' + response.data.errors[0].extensions.internal_error
          console.log('Server error retrieving environments: ' + errmsg)
          dispatch('notifications/addNotification',
            {
              title: 'Server Error retrieving environments',
              message: errmsg,
              theme: 'error',
              timeout: 5000
            },
            { root: true }
          )
        }
        const environments = response.data.data.environments
        commit('updateEnvironments', environments)
      })
    } catch (err) {
      console.log('retrieving environments error: ' + err)
      dispatch('notifications/addNotification',
        {
          title: 'Error retrieving environments',
          message: err,
          theme: 'error',
          timeout: 5000
        },
        { root: true }
      )
    }
  },
  // loads the indexes of an environment specified by 'id'
  loadIndexes: async ({ dispatch, commit }, { id }) => {
    const variables = {
      id: id
    }
    const query = `query indexes($id: Uuid!) {
      indexes(id: $id) {
        id,
        signature,
        indexType,
        dataSource,
        regions,
        filepath,
        status,
        createdAt,
        updatedAt
      }
    }`

    try {
      await axios({
        method: 'post',
        headers: {
          Accept: 'application/json',
          'Content-Type': 'application/json'
        },
        url: ApiRoutes.GraphQL,
        data: JSON.stringify({
          query: query,
          variables: variables
        })
      }).then(response => {
        if (response.data.errors) {
          const errmsg = response.data.errors.message
          dispatch('notifications/addNotification',
            {
              title: 'Server Error retrieving indexes',
              message: errmsg,
              theme: 'error',
              timeout: 5000
            },
            { root: true }
          )
        }
        const indexes = response.data.data.indexes
        commit('updateEnvironmentIndexes', { id, indexes })
      })
    } catch (err) {
      console.log('Retrieving Indexes error: ' + err)
      dispatch('notifications/addNotification',
        {
          title: 'Error retrieving indexes',
          message: err,
          theme: 'error',
          timeout: 5000
        },
        { root: true }
      )
    }
  }
}

export default {
  namespaced: true,
  state,
  getters,
  actions,
  mutations
}
