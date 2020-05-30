<template>
  <div id="app">
    <router-view/>
    <Notifications />
  </div>
</template>

<script>
import Notifications from '@/components/Util/Notifications'
import eventSource from '@/api/eventSource'

export default {
  name: 'App',
  components: {
    Notifications
  },
  async created () {
    eventSource.configure()
  }
}
</script>
