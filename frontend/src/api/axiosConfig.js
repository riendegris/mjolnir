import axios from 'axios'
import store from '@/store'

export default function configureAxios () {
  axios.interceptors.response.use(undefined, function (err) {
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
