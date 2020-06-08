import axios from 'axios'
import ApiRoutes from '@/api/apiRoutes'

const state = {
  features: []
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
  }
}

// FIXME: I switched notation halfway through, going from 'id' to 'feature', or 'scenario'.
// Go back and change notation so that its the same everywhere.
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
  // Update the environment of the background of the feature identified by 'id'
  // FIXME Need protection against undefined background
  updateBackgroundEnvironment: (state, { id, environment }) => {
    const i = state.features.findIndex(obj => obj.id === id)
    state.features[i].background.environment = environment
  },
  // FIXME Need protection against undefined
  updateScenarioSteps: (state, { feature, scenario, steps }) => {
    const i = state.features.findIndex(obj => obj.id === feature)
    const j = state.features[i].scenarios.findIndex(obj => obj.id === scenario)
    state.features[i].scenarios[j].steps = steps
  }
}

const actions = {
  loadFeatures: async ({ dispatch, commit }) => {
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
          console.log('Server error retrieving features: ' + errmsg)
          dispatch('notifications/addNotification',
            {
              title: 'Server Error retrieving features',
              message: errmsg,
              theme: 'error',
              timeout: 5000
            },
            { root: true }
          )
        }
        const features = response.data.data.features
        commit('updateFeatures', features)
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
          const errmsg = response.data.errors[0].message + ': ' + response.data.errors[0].extensions.internal_error
          console.log('Server Error uploading feature: ' + errmsg)
          dispatch('notifications/addNotification',
            {
              title: 'Server Error uploading feature',
              message: errmsg,
              theme: 'error',
              timeout: 5000
            },
            { root: true }
          )
        }
        // We're not commiting this feature, because we are
        // also subscribed to feature notifications, and so
        // we should also get the information from there.
        // const feature = response.data.data.loadFeature
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
          const errmsg = response.data.errors[0].message + ': ' + response.data.errors[0].extensions.internal_error
          console.log('Server Error retrieving scenarios: ' + errmsg)
          dispatch('notifications/addNotification',
            {
              title: 'Server Error retrieving scenarios',
              message: errmsg,
              theme: 'error',
              timeout: 5000
            },
            { root: true }
          )
        }
        const scenarios = response.data.data.scenarios
        commit('updateFeatureScenarios', { id, scenarios })
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
    }
  },
  loadBackground: async ({ state, dispatch }, { id }) => {
    await dispatch('loadBackgroundCore', { id })
    const i = state.features.findIndex(obj => obj.id === id)
    const background = state.features[i].background
    if (background !== undefined) {
      await dispatch('loadBackgroundSteps', { feature: id, id: state.features[i].background.id })
      await dispatch('loadBackgroundEnvironment', { feature: id, id: state.features[i].background.id })
    }
  },

  // Load the background of feature 'id'
  loadBackgroundCore: async ({ dispatch, commit }, { id }) => {
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
          const errmsg = response.data.errors[0].message + ': ' + response.data.errors[0].extensions.internal_error
          console.log('Server Error retrieving background: ' + errmsg)
          dispatch('notifications/addNotification',
            {
              title: 'Server Error retrieving background',
              message: errmsg,
              theme: 'error',
              timeout: 5000
            },
            { root: true }
          )
        }
        const background = response.data.data.background
        if (background !== null && background !== undefined) {
          console.log('there is a background')
          commit('updateBackground', { id, background })
        } else {
          console.log('there is no background')
        }
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
    }
  },
  // Load the steps of background 'id'
  loadBackgroundSteps: async ({ dispatch, commit }, { feature, id }) => {
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
          const errmsg = response.data.errors[0].message + ': ' + response.data.errors[0].extensions.internal_error
          console.log('Server Error retrieving background steps: ' + errmsg)
          dispatch('notifications/addNotification',
            {
              title: 'Server Error retrieving steps',
              message: errmsg,
              theme: 'error',
              timeout: 5000
            },
            { root: true }
          )
        }
        const steps = response.data.data.steps
        commit('updateBackgroundSteps', { id: feature, steps })
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
    }
  },
  // Load the environment of background 'id'
  loadBackgroundEnvironment: async ({ dispatch, commit }, { feature, id }) => {
    const variables = {
      id: id
    }
    const query = `query environment($id: Uuid!) {
      environment(id: $id, src: BACKGROUND) {
        id
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
          const errmsg = response.data.errors[0].message + ': ' + response.data.errors[0].extensions.internal_error
          console.log('Server Error retrieving background environment: ' + errmsg)
          dispatch('notifications/addNotification',
            {
              title: 'Server Error retrieving environment',
              message: errmsg,
              theme: 'error',
              timeout: 5000
            },
            { root: true }
          )
        }
        const environment = response.data.data.environment
        commit('updateBackgroundEnvironment', { id: feature, environment })
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
    }
  },
  // Load the steps of scenario 'id'
  loadScenarioSteps: async ({ dispatch, commit }, { feature, scenario }) => {
    console.log('loading steps for scenario ' + scenario)
    const variables = {
      id: scenario
    }
    const query = `query steps($id: Uuid!) {
      steps(id: $id, src: SCENARIO) {
        id,
        stepType,
        value,
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
        }
        commit('updateScenarioSteps', { feature, scenario, steps })
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
