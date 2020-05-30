<template>
  <header class="header flex align-center justify-center px-0 py-0 bg-blue-900">
    <button v-for="route in navigationRouteOptions" v-bind:key="route.name" @click="goTo(route.path)"
      class="inline-block py-1 md:py-4 text-gray-500 hover:text-gray-100 font-semibold tracking-wide mr-6">
      {{ route.name }}
    </button>
  </header>
</template>

<script>
import AppRoutes from '@/router'
export default {
  name: 'Header',
  data () {
    return {
      navigationRouteOptions: [AppRoutes.Home, AppRoutes.Dashboard]
    }
  },
  methods: {
    goBack () {
      this.$router.back()
    },
    goHome () {
      this.$router.push(AppRoutes.Home)
    },
    goTo (routeName) {
      this.$router.push(routeName)
    }
  }
}
</script>

<style>
.header {
  grid-area: header;
}
</style>
