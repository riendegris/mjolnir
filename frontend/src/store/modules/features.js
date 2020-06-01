import axios from 'axios'
import ApiRoutes from '@/api/apiRoutes'

const state = {
  features: []
}

const getters = {
  features: state => state.features,
  feature: (state, getters) => (id) => {
    const i = state.features.findIndex(obj => obj.id === id)
    return state.features[i]
  }
}

const mutations = {
  updateFeatures: (state, features) => { state.features = features },
  // Feature upsert: insert if not present, update otherwise.
  updateFeature: (state, { id, name, description, tags, createdAt, updatedAt }) => {
    const i = state.features.findIndex(obj => obj.id === id)
    const feature = { id, name, description, tags, createdAt, updatedAt }
    if (i === -1) { // not found
      state.features.push(feature)
    } else {
      state.features.splice(i, 1, feature)
    }
  }
}

const actions = {
  loadFeatures: async ({ dispatch, commit }) => {
    if (process.env.NODE_ENV === 'test') {
      axios.get('/data/features.json')
        .then(response => {
          const features = response.data
          console.log(features)
          commit('updateFeatures', features)
        })
    } else {
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
          }
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
  }
}

export default {
  namespaced: true,
  state,
  getters,
  actions,
  mutations
}
