import store from '@/store'

const eventSource = {
  connection: null,
  configure: function () {
    console.log('Starting connection to WebSocket Server')
    this.connection = new WebSocket('ws://localhost:3030/notifications')
    this.connection.onerror = this.onError
    this.connection.onopen = this.onConnectionOpen
    this.connection.onmessage = this.onMessage
  },
  onError: function (event) {
    console.log('Error w/ Websocket')
  },
  onConnectionOpen: function (event) {
    console.log(event)
    console.log('Successfully connected to stocks notifications')
    const message = `{
      "id": "1",
      "type":"start",
      "payload": {
          "query": "subscription stocks { stocks { name, quantity, price } }"
        }
    }`
    this.send(message)
  },
  onMessage: function (event) {
    console.log(event)
    try {
      const json = JSON.parse(event.data)
      console.log(json)
      store.commit('stocks/updateStock', json.payload.data.stocks)
    } catch (err) {
      store.dispatch('notifications/addNotification',
        {
          title: 'Error stock notification',
          message: err,
          theme: 'error',
          timeout: 3000
        },
        { root: true }
      )
    }
  },
  teardown: function () {
    this.connection.close()
  }
}

export default eventSource
