<template>
  <div class='fixed right-0 top-0 z-50'>
    <transition-group name='notification'>
      <div v-for='notification in notifications' :key='notification.key' class='shadow-lg rounded-lg bg-white mx-auto m-8 p-4'>
        <div class='text-sm pb-2'>
          {{ notification.title }}
        </div>

        <div @click.prevent="remove(notification.key)" class="float-right cursor-pointer select-none">
          <!-- icon from feathericons.com -->
          <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" class="feather feather-x-circle"><circle cx="12" cy="12" r="10"></circle><line x1="15" y1="9" x2="9" y2="15"></line><line x1="9" y1="9" x2="15" y2="15"></line></svg>
        </div>
        <div class='text-sm text-gray-600 tracking-tight'>
          {{ notification.message }}
        </div>
      </div>
    </transition-group>
  </div>
</template>

<script>
import { mapGetters, mapActions } from 'vuex'

export default {
  computed: {
    ...mapGetters({
      notifications: 'notifications/notifications'
    })
  },
  methods: {
    ...mapActions({
      remove: 'notifications/removeNotification'
    })
  }
}
</script>
