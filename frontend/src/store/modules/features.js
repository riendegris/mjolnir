import axios from 'axios'
import ApiRoutes from '@/api/apiRoutes'

const state = {
  features: [],
  featuresLoading: false,
  scenariosLoading: false,
  backgroundLoading: false,
  scenarioLoading: false
}

const getters = {
  features: state => state.features,
  feature: state => (id) => {
    const i = state.features.findIndex(obj => obj.id === id)
    return state.features[i]
  },
  scenarios: state => (id) => {
    const i = state.features.findIndex(obj => obj.id === id)
    if (typeof state.features[i].scenarios === 'undefined') {
      return []
    }
    return state.features[i].scenarios
  },
  background: state => (id) => {
    const i = state.features.findIndex(obj => obj.id === id)
    return state.features[i].background
  },
  featuresLoading: state => state.featuresLoading,
  scenariosLoading: state => state.scenariosLoading,
  backgroundLoading: state => state.backgroundLoading,
  scenarioLoading: state => state.scenarioLoading
}

const mutations = {
  updateFeatures: (state, features) => { state.features = features },
  // Update the scenarios of the feature identified by 'id'
  updateFeatureScenarios: (state, { id, scenarios }) => {
    const i = state.features.findIndex(obj => obj.id === id)
    state.features[i].scenarios = scenarios
  },
  // Update the background of the feature identified by 'id'
  updateBackground: (state, { id, background }) => {
    const i = state.features.findIndex(obj => obj.id === id)
    state.features[i].background = background
  },
  // Update the steps of the background of the feature identified by 'id'
  // FIXME Need protection against undefined background
  updateBackgroundSteps: (state, { id, steps }) => {
    const i = state.features.findIndex(obj => obj.id === id)
    state.features[i].background.steps = steps
  },
  featuresLoading: (state) => { state.featuresLoading = true },
  scenariosLoading: (state) => { state.scenariosLoading = true },
  scenarioLoading: (state) => { state.scenarioLoading = true },
  backgroundLoading: (state) => { state.backgroundLoading = true },
  featuresReady: (state) => { state.featuresLoading = false },
  scenariosReady: (state) => { state.scenariosLoading = false },
  scenarioReady: (state) => { state.scenarioLoading = false },
  backgroundReady: (state) => { state.backgroundLoading = false }
}

const actions = {
  loadFeatures: async ({ dispatch, commit }) => {
    commit('featuresLoading')
    const query = `{
      features {
        id,
        name,
        description,
        tags
        createdAt,
        updatedAt
      }
    }`

    try {
      axios({
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
        console.log(response)
        const features = response.data.data.features
        if (features.error) {
          dispatch('notifications/addNotification',
            {
              title: 'Server Error retrieving features',
              message: features.error,
              theme: 'error',
              timeout: 5000
            },
            { root: true }
          )
          commit('featuresReady')
        }
        commit('updateFeatures', features)
        commit('featuresReady')
      })
    } catch (err) {
      console.log('retrieving Features error: ' + err)
      dispatch('notifications/addNotification',
        {
          title: 'Error retrieving features',
          message: err,
          theme: 'error',
          timeout: 5000
        },
        { root: true }
      )
      commit('featuresReady')
    }
  },
  uploadFeature: async ({ dispatch, commit }, { text }) => {
    const variables = {
      feature: text
    }
    const query = ` mutation loadFeature($feature: String!) {
      loadFeature(feature: $feature) {
        id,
        name,
        description,
        tags
        createdAt,
        updatedAt
      }
    }`

    try {
      axios({
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
        const feature = response.data.data.loadFeature
        if (feature.error) {
          dispatch('notifications/addNotification',
            {
              title: 'Server Error uploading feature',
              message: feature.error,
              theme: 'error',
              timeout: 5000
            },
            { root: true }
          )
        }
        // We're not commiting this feature, because we are
        // also subscribed to feature notifications, and so
        // we should also get the information from there.
        // commit('updateFeature', feature)
      })
    } catch (err) {
      console.log('uploading feature error: ' + err)
      dispatch('notifications/addNotification',
        {
          title: 'Error uploading feature',
          message: err,
          theme: 'error',
          timeout: 5000
        },
        { root: true }
      )
    }
  },
  loadScenarios: async ({ dispatch, commit }, { id }) => {
    commit('scenariosLoading')
    const variables = {
      id: id
    }
    const query = `query scenarios($id: Uuid!) {
      scenarios(id: $id) {
        id,
        name,
        tags
        createdAt,
        updatedAt
      }
    }`

    try {
      axios({
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
        const scenarios = response.data.data.scenarios
        if (scenarios.error) {
          dispatch('notifications/addNotification',
            {
              title: 'Server Error retrieving scenarios',
              message: scenarios.error,
              theme: 'error',
              timeout: 5000
            },
            { root: true }
          )
          commit('scenariosReady')
        }
        commit('updateFeatureScenarios', { id, scenarios })
        commit('scenariosReady')
      })
    } catch (err) {
      console.log('Retrieving Scenarios error: ' + err)
      dispatch('notifications/addNotification',
        {
          title: 'Error retrieving scenarios',
          message: err,
          theme: 'error',
          timeout: 5000
        },
        { root: true }
      )
      commit('scenariosReady')
    }
  },
  // Load the background of feature 'id'
  loadBackground: async ({ dispatch, commit }, { id }) => {
    commit('backgroundLoading')
    const variables = {
      id: id
    }
    const query = `query background($id: Uuid!) {
      background(id: $id) {
        id,
        createdAt,
        updatedAt
      }
    }`
    try {
      axios({
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
        const background = response.data.data.background
        if (background.error) {
          dispatch('notifications/addNotification',
            {
              title: 'Server Error retrieving background',
              message: background.error,
              theme: 'error',
              timeout: 5000
            },
            { root: true }
          )
          commit('backgroundReady')
        }
        commit('updateBackground', { id, background })
        dispatch('loadBackgroundSteps', { feature: id, id: background.id })
        commit('backgroundReady')
      })
    } catch (err) {
      console.log('Retrieving Background error: ' + err)
      dispatch('notifications/addNotification',
        {
          title: 'Error retrieving background',
          message: err,
          theme: 'error',
          timeout: 5000
        },
        { root: true }
      )
      commit('backgroundReady')
    }
  },
  // Load the steps of background 'id'
  loadBackgroundSteps: async ({ dispatch, commit }, { feature, id }) => {
    commit('backgroundLoading')
    const variables = {
      id: id
    }
    const query = `query steps($id: Uuid!) {
      steps(id: $id, src: BACKGROUND) {
        id,
        stepType,
        value,
        createdAt,
        updatedAt
      }
    }`

    try {
      axios({
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
        console.log(response)
        const steps = response.data.data.steps
        if (steps.error) {
          dispatch('notifications/addNotification',
            {
              title: 'Server Error retrieving steps',
              message: steps.error,
              theme: 'error',
              timeout: 5000
            },
            { root: true }
          )
          commit('backgroundReady')
        }
        commit('updateBackgroundSteps', { id: feature, steps })
        commit('backgroundReady')
      })
    } catch (err) {
      console.log('Retrieving Steps error: ' + err)
      dispatch('notifications/addNotification',
        {
          title: 'Error retrieving steps',
          message: err,
          theme: 'error',
          timeout: 5000
        },
        { root: true }
      )
      commit('backgroundReady')
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
