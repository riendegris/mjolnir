import axios from 'axios'
import ApiRoutes from '@/api/apiRoutes'

const state = {
  banos: []
}

const getters = {
  banos: state => state.banos
}

const mutations = {
  updateBanos: (state, list) => { state.banos = list },
  addBano: (state, item) => { state.banos.push(item) },
  addBanoItem: (state, { id, item }) => {
    const i = state.banos.findIndex(obj => obj.id === id)
    state.banos[i].items.push(item)
  },
  removeBanoItem: (state, { id, iid }) => {
    const i = state.banos.findIndex(obj => obj.id === id)
    const j = state.banos[i].items.findIndex(obj => obj.id === iid)
    state.banos[i].items.splice(j, 1)
  },
  updateBanoItem: (state, { id, item }) => {
    const i = state.banos.findIndex(obj => obj.id === id)
    const bano = state.banos[i]
    const j = bano.items.findIndex(obj => obj.id === item.id)
    const items = bano.items
    items.splice(j, 1, item)
    const newbano = { ...bano, items: items }
    state.banos.splice(i, 1, newbano)
  }
}

const actions = {
  loadBanos: async ({ dispatch, commit }) => {
    if (process.env.NODE_ENV === 'test') {
      axios.get('/data/banos.json')
        .then(response => {
          const banos = response.data
          console.log(banos)
          commit('updateBanos', banos)
        })
    } else {
      const query = `{
        banos {
          error,
          data {
            id,
            description,
            items { id, filename, md5, filesize, filestatus, updatedAt }
          }
        }
      }`

      console.log('retrieving banos')

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
          const resp = response.data.data.banos
          if (resp.error) {
            dispatch('notifications/addNotification',
              {
                title: 'Server Error retrieving banos',
                message: resp.error,
                theme: 'error',
                timeout: 3000
              },
              { root: true }
            )
          }
          commit('updateBanos', resp.data)
        })
      } catch (err) {
        console.log('retrieving banos error: ' + err)
        dispatch('notifications/addNotification',
          {
            title: 'Error retrieving banos',
            message: err,
            theme: 'error',
            timeout: 3000
          },
          { root: true }
        )
      }
    }
  },

  addBano: async ({ dispatch, commit }, { id, description }) => {
    const variables = {
      id: id,
      desc: description
    }
    const query = `mutation addBano($id: String!, $desc: String!) {
      addBano(banoId: $id, description: $desc) {
        error,
        data { id, description }
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
        const resp = response.data.data.addBano
        if (resp.error) {
          console.log('dispatching because not null')
          dispatch('notifications/addNotification',
            {
              title: 'Server error adding bano',
              message: resp.error,
              theme: 'error',
              timeout: 3000
            },
            { root: true }
          )
        }
        commit('addBano', resp.data)
      })
    } catch (err) {
      dispatch('notifications/addNotification',
        {
          title: 'Error adding bano',
          message: err,
          theme: 'error',
          timeout: 3000
        },
        { root: true }
      )
    }
  },

  addBanoItem: async ({ dispatch, commit }, { id, iid }) => {
    const variables = {
      id: id,
      iid: iid
    }
    const query = `mutation addBanoItem($id: String!, $iid: String!) {
      addBanoItem(banoId: $id, itemId: $iid) {
        error,
        data { id, filename, md5, filesize, filestatus, updatedAt }
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
        const resp = response.data.data.addBanoItem
        console.log(resp)
        if (resp.error) {
          console.log('dispatching because not null')
          dispatch('notifications/addNotification',
            {
              title: 'Server error adding bano item',
              message: resp.error,
              theme: 'error',
              timeout: 3000
            },
            { root: true }
          )
        }
        const item = resp.data
        commit('addBanoItem', { id, item })
      })
    } catch (err) {
      dispatch('notifications/addNotification',
        {
          title: 'Error adding bano item',
          message: err,
          theme: 'error',
          timeout: 3000
        },
        { root: true }
      )
    }
  },

  removeBanoItem: async ({ dispatch, commit }, { id, iid }) => {
    const variables = {
      id: id,
      iid: iid
    }
    const query = `mutation removeBanoItem($id: String!, $iid: String!) {
      removeBanoItem(banoId: $id, itemId: $iid) {
        error
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
        const resp = response.data.data.removeBanoItem
        console.log(resp)
        if (resp.error) {
          console.log('dispatching because not null')
          dispatch('notifications/addNotification',
            {
              title: 'Server error adding bano item',
              message: resp.error,
              theme: 'error',
              timeout: 3000
            },
            { root: true }
          )
        }
        commit('removeBanoItem', { id, iid })
      })
    } catch (err) {
      dispatch('notifications/addNotification',
        {
          title: 'Error adding bano item',
          message: err,
          theme: 'error',
          timeout: 3000
        },
        { root: true }
      )
    }
  },

  downloadBanoItem: async ({ dispatch, commit }, { id, iid }) => {
    const variables = {
      id: id,
      iid: iid
    }
    const query = `mutation downloadBanoItem($id: String!, $iid: String!) {
      downloadBanoItem(banoId: $id, itemId: $iid) {
        error,
        data { id, filename, md5, filesize, filestatus, updatedAt }
      }
    }`

    try {
      axios({
        method: 'post',
        headers: {
          Accept: 'application/json',
          'Content-Type': 'application/json'
        },
        url: '/graphql',
        data: JSON.stringify({
          query: query,
          variables: variables
        })
      }).then(response => {
        const resp = response.data.data.downloadBanoItem
        console.log(resp)
        if (resp.error) {
          console.log('dispatching because not null')
          dispatch('notifications/addNotification', 'Error adding bano item', resp.error, 'error', 3000, { root: true })
        }
        const item = resp.data
        commit('updateBanoItem', { id, item })
      })
    } catch (err) {
      dispatch('notifications/addNotification', 'Error adding bano item', err, 'error', 3000, { root: true })
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
