import axios from 'axios'
import store from '@/store'

export default function configureAxios () {
  let callPending = 0
  axios.interceptors.request.use(function (config) {
    callPending++
    store.dispatch('loader/show')
    return config
  }, function (err) {
    return Promise.reject(err)
  })
  axios.interceptors.response.use(function (response) {
    callPending--
    if (callPending === 0) {
      store.dispatch('loader/hide')
    }
    return response
  }, function (err) {
    callPending--
    if (callPending === 0) {
      store.dispatch('loader/hide')
    }
    store.dispatch('notifications/addNotification',
      {
        title: 'Axios Error',
        message: err,
        theme: 'error',
        timeout: 3000
      },
      { root: true }
    )
    return Promise.reject(err)
  })
}
