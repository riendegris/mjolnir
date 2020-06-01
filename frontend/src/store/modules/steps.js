import axios from 'axios'
import ApiRoutes from '@/api/apiRoutes'

const state = {
  steps: []
}

const getters = {
  steps: state => state.steps
}

const mutations = {
  updateSteps: (state, steps) => { state.steps = steps },
  // Step upsert: insert if not present, update otherwise.
  updateStep: (state, { id, name, tags, createdAt, updatedAt }) => {
    const i = state.steps.findIndex(obj => obj.id === id)
    const step = { id, name, tags, createdAt, updatedAt }
    if (i === -1) { // not found
      state.steps.push(step)
    } else {
      state.steps.splice(i, 1, step)
    }
  }
}

const actions = {
  loadSteps: async ({ dispatch, commit }, { id }) => {
    if (process.env.NODE_ENV === 'test') {
      axios.get('/data/steps.json')
        .then(response => {
          const steps = response.data
          console.log(steps)
          commit('updateSteps', steps)
        })
    } else {
      console.log('retrieving steps with scenario ' + id)
      const variables = {
        id: id
      }
      const query = `query steps($id: Uuid!) {
        steps(id: $id) {
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
          }
          commit('updateSteps', steps)
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
}

export default {
  namespaced: true,
  state,
  getters,
  actions,
  mutations
}
