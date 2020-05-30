import Vue from 'vue'
import Router from 'vue-router'

import Home from '@/components/Home/Home'
import Dashboard from '@/components/Dashboard/Dashboard'

Vue.use(Router)

const AppRoutes = {
  Home: {
    path: '/',
    name: 'Home',
    component: Home
  },
  Dashboard: {
    path: '/dashboard',
    name: 'Dashboard',
    component: Dashboard
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
