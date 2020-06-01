import axios from 'axios'
import ApiRoutes from '@/api/apiRoutes'

const state = {
  scenarios: []
}

const getters = {
  scenarios: state => state.scenarios
}

const mutations = {
  updateScenarios: (state, scenarios) => { state.scenarios = scenarios },
  // Scenario upsert: insert if not present, update otherwise.
  updateScenario: (state, { id, name, tags, createdAt, updatedAt }) => {
    const i = state.scenarios.findIndex(obj => obj.id === id)
    const scenario = { id, name, tags, createdAt, updatedAt }
    if (i === -1) { // not found
      state.scenarios.push(scenario)
    } else {
      state.scenarios.splice(i, 1, scenario)
    }
  }
}

const actions = {
  loadScenarios: async ({ dispatch, commit }, { id }) => {
    if (process.env.NODE_ENV === 'test') {
      axios.get('/data/scenarios.json')
        .then(response => {
          const scenarios = response.data
          console.log(scenarios)
          commit('updateScenarios', scenarios)
        })
    } else {
      console.log('retrieving scenarios with feature ' + id)
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
          console.log(response)
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
          }
          commit('updateScenarios', scenarios)
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
