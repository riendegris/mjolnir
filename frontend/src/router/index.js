import Vue from 'vue'
import Router from 'vue-router'

import Home from '@/components/Home/Home'
import Banos from '@/components/Banos/Banos'

Vue.use(Router)

const AppRoutes = {
  Home: {
    path: '/',
    name: 'Home',
    component: Home
  },
  Banos: {
    path: '/banos',
    name: 'Banos',
    component: Banos
  }
}

export const AppRouter = new Router({
  mode: 'history',
  base: process.env.BASE_URL, /* FIXME maybe not necessary and creates interference */
  routes: Object.values(AppRoutes)
})

AppRouter.beforeEach((to, from, next) => {
  document.title = `Mjolnir: ${to.name}`
  next()
})

export default AppRoutes
