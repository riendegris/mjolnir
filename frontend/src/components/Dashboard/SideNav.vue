<template>
  <aside class="sidenav bg-blue-800 flex flex-col">
    <ul class="font-header p-0 mt-16 list-none">
      <li class="p-8 text-gray-500">
        <span class="uppercase text-gray-500">Features</span>
        <ul class="pl-4 pt-2">
          <li v-for="feature in features" :key="feature.id">
            <button class="text-gray-400" @click='switchPanel("Features")'>{{ feature.name }}</button>
          </li>
        </ul>
      </li>
      <li class="p-8 text-gray-500">
        <span class="uppercase text-gray-500">Runs</span>
        <ul class="pl-4 pt-2">
          <li>
            <button class="text-gray-400 text-green-500" @click='switchPanel("Runs")'>May 25th, 10:51</button>
          </li>
          <li>
            <button class="text-gray-400 text-red-500" @click='switchPanel("Runs")'>May 24th, 10:51</button>
          </li>
        </ul>
      </li>
      <li class="p-8 text-gray-500">
        <span class="uppercase text-gray-500">Environments</span>
        <ul class="pl-4 pt-2">
          <li>
            <button class="text-gray-400 text-green-500" @click='switchPanel("Environment")'>France</button>
          </li>
          <li>
            <button class="text-gray-400 text-red-500" @click='switchPanel("Runs")'>Ile-de-France</button>
          </li>
          <li>
            <button class="text-gray-400 text-red-500" @click='switchPanel("Runs")'>Netherlands</button>
          </li>
        </ul>
      </li>
      <li class="sidenav__list-item">Item Three</li>
      <li class="sidenav__list-item">Item Four</li>
      <li class="sidenav__list-item">Item Five</li>
    </ul>
  </aside>
</template>

<script>
import { mapGetters, mapActions } from 'vuex'

export default {
  name: 'SideNav',
  computed: {
    ...mapGetters({
      features: 'features/features'
    })
  },
  methods: {
    ...mapActions({
      switchPanel: 'dashboard/switchPanel',
      loadFeatures: 'features/loadFeatures'
    })
  },
  async created () {
    this.loadFeatures()
  }
}
</script>

<style>
.sidenav {
  grid-area: sidenav;
}

.sidenav__list-item {
  padding: 20px 20px 20px 40px;
  color: #ddd;
}

.sidenav__list-item:hover {
  background-color: rgba(255, 255, 255, 0.2);
  cursor: pointer;
}
</style>
