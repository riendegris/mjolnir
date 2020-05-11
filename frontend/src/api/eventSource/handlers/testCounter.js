import store from '@/store'

const nodeStatus = {
  eventType: 'TEST_COUNTER',
  handle: function (event) {
    try {
      const json = JSON.parse(event.data)
      store.commit('test/updateCounter', json.counter)
    } catch (err) {
      store.dispatch('notifications/addNotification',
        {
          title: 'Error updating test counter',
          message: err,
          theme: 'error',
          timeout: 3000
        },
        { root: true }
      )
    }
  }
}

export default nodeStatus
