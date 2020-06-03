import axios from 'axios'
import ApiRoutes from '@/api/apiRoutes'

const state = {
  backgrounds: []
}

const getters = {
  backgrounds: state => state.backgrounds
}

const mutations = {
  updateBackgrounds: (state, backgrounds) => { state.backgrounds = backgrounds },
  // Background upsert: insert if not present, update otherwise.
  updateBackground: (state, { id, name, tags, createdAt, updatedAt }) => {
    const i = state.backgrounds.findIndex(obj => obj.id === id)
    const background = { id, name, tags, createdAt, updatedAt }
    if (i === -1) { // not found
      state.backgrounds.push(background)
    } else {
      state.backgrounds.splice(i, 1, background)
    }
  }
}

const actions = {
  loadBackgrounds: async ({ dispatch, commit }, { id }) => {
    if (process.env.NODE_ENV === 'test') {
      axios.get('/data/backgrounds.json')
        .then(response => {
          const backgrounds = response.data
          console.log(backgrounds)
          commit('updateBackgrounds', backgrounds)
        })
    } else {
      console.log('retrieving backgrounds with feature ' + id)
      const variables = {
        id: id
      }
      const query = `query backgrounds($id: Uuid!) {
        backgrounds(id: $id) {
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
          const backgrounds = response.data.data.backgrounds
          if (backgrounds.error) {
            dispatch('notifications/addNotification',
              {
                title: 'Server Error retrieving backgrounds',
                message: backgrounds.error,
                theme: 'error',
                timeout: 5000
              },
              { root: true }
            )
          }
          commit('updateBackgrounds', backgrounds)
        })
      } catch (err) {
        console.log('Retrieving Backgrounds error: ' + err)
        dispatch('notifications/addNotification',
          {
            title: 'Error retrieving backgrounds',
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
