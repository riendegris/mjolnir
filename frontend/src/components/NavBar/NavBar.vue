<template>
  <div class="bg-blue-800 px-4 py-4">
    <div
      class="md:max-w-6xl md:mx-auto md:flex md:items-center md:justify-between"
    >
      <div class="flex justify-between items-center">
        <a href="#" class="inline-block py-2 text-white text-xl font-bold"
          >Mjolnir</a
        >
        <div
          class="inline-block cursor-pointer md:hidden">
          <div class="bg-gray-400 w-8 mb-2" style="height: 2px;"></div>
          <div class="bg-gray-400 w-8 mb-2" style="height: 2px;"></div>
          <div class="bg-gray-400 w-8" style="height: 2px;"></div>
        </div>
      </div>

      <div>
        <div class="hidden md:block">
          <!-- This is what goes in the center -->
          <button v-for="route in navigationRouteOptions" v-bind:key="route.name" @click="goTo(route.path)"
            class="inline-block py-1 md:py-4 text-gray-500 hover:text-gray-100 mr-6">
            {{ route.name }}
          </button>
        </div>
      </div>
      <div class="hidden md:block">
        <!-- This is what goes on the right hand side -->
      </div>
    </div>
  </div>
</template>

<script>
import AppRoutes from '@/router'
export default {
  name: 'NavBar',
  data () {
    return {
      navigationRouteOptions: [AppRoutes.Home, AppRoutes.Banos]
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
