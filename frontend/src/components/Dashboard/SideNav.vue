<template>
  <aside class="sidenav bg-blue-800 flex flex-col">
    <ul class="font-header p-0 mt-16 list-none">
      <li class="p-8 text-gray-500">
        <div class="flex justify-between">
          <span class="uppercase text-gray-500 font-header">Features</span>
          <button @click.prevent='switchPanel({"panel": "NewFeature", "key": "feature"})' class="rounded-full border border-gray-500 w-7 h-7 flex items-center justify-center">
            <!-- icon by feathericons.com: + -->
            <svg aria-hidden="true" class="" fill="none" height="24" stroke="#a0aec0" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" viewbox="0 0 24 24" width="24" xmlns="http://www.w3.org/2000/svg">
              <line x1="12" y1="8" x2="12" y2="16"></line><line x1="8" y1="12" x2="16" y2="12"></line>
            </svg>
          </button>
        </div>
        <ul class="pl-4 pt-2">
          <!--<li>
            <button class="text-gray-400 text-sm" @click='switchPanel("Features")'>{{ truncate('searching for POI in a far away galaxy', 25, true) }}</button>
          </li>-->
          <li v-for="feature in features" :key="feature.id">
            <button class="text-gray-400 text-sm font-header" @click='switchPanel({"panel": "Feature", "key": "id", "value": feature.id })'>{{ truncate(feature.name, 22, true) }}</button>
          </li>
        </ul>
      </li>
      <li class="p-8 text-gray-500">
        <span class="uppercase text-gray-500">Environments</span>
        <ul class="pl-4 pt-2">
          <li v-for="environment in environments" :key="environment.id">
            <button class="text-gray-400 text-sm font-header" @click='switchPanel({"panel": "Environment", "key": "id", "value": environment.id })'>{{ truncate(environment.signature, 22, true) }}</button>
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
    </ul>
  </aside>
</template>

<script>
import { mapGetters, mapActions } from 'vuex'

export default {
  name: 'SideNav',
  computed: {
    ...mapGetters({
      features: 'features/features',
      environments: 'environments/environments'
    })
  },
  methods: {
    ...mapActions({
      switchPanel: 'dashboard/switchPanel',
      loadFeatures: 'features/loadFeatures',
      loadEnvironments: 'environments/loadEnvironments'
    })
  },
  async created () {
    await this.loadFeatures()
    await this.loadEnvironments()
  }
}
</script>

<style>
.sidenav {
  grid-area: sidenav;
}
</style>
