<template>
  <div id="app">
    <NavBar/>
    <router-view/>
    <Notifications />
  </div>
</template>

<script>
import NavBar from '@/components/NavBar/NavBar'
import Notifications from '@/components/Util/Notifications'

export default {
  name: 'App',
  components: {
    NavBar,
    Notifications
  }
}
</script>
