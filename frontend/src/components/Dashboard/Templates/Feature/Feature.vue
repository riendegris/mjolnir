<template>
  <div v-if="foo" class="feature-panel flex-grow">
    <Scenarios :scenarios='scenarios' :feature='feature' @selectIndex='selectScenario'/>
    <Scenario :scenario='scenario' />
    <Background v-if="hasBackground" :background='background' />
  </div>
  <div v-else>
    <p>Loading</p>
  </div>
</template>

<script>
import { mapGetters, mapActions } from 'vuex'
import Scenarios from './Scenarios'
import Scenario from './Scenario'
import Background from './Background'

export default {
  name: 'Feature',
  components: {
    Scenarios,
    Scenario,
    Background
  },
  data () {
    return {
      idx: 0, // index of the scenario selected by the user, drives the Scenario component
      // it is initialized to 0 here, but it is driven by the Scenarios component.
      // FIXME Get a more descriptive variable name
      foo: false,
      scenario: {}
    }
  },
  computed: {
    ...mapGetters({
      id: 'dashboard/value'
    }),
    feature () {
      return this.$store.getters['features/feature'](this.id)
    },
    scenarios () {
      return this.$store.getters['features/scenarios'](this.id)
    },
    background () {
      return this.$store.getters['features/background'](this.id)
    },
    hasBackground () {
      return this.background !== null || this.background !== undefined
    }
  },
  methods: {
    ...mapActions({
      loadScenarios: 'features/loadScenarios',
      loadBackground: 'features/loadBackground',
      loadScenarioSteps: 'features/loadScenarioSteps'
    }),
    async selectScenario (idx) {
      this.idx = idx
      const scenario = this.$store.getters['features/scenarios'](this.id)[this.idx]
      await this.loadScenarioSteps({feature: this.id, scenario: scenario.id})
      this.scenario = this.$store.getters['features/scenarios'](this.id)[this.idx]
    }
  },
  async created () {
    await this.loadScenarios({ id: this.id })
    const scenario = this.$store.getters['features/scenarios'](this.id)[this.idx]
    await this.loadBackground({ id: this.id })
    console.log('scenario id: ' + scenario.id)
    await this.loadScenarioSteps({feature: this.id, scenario: scenario.id})
    this.scenario = this.$store.getters['features/scenarios'](this.id)[this.idx]
    this.foo = true
  }
}
</script>

<style>
.feature-panel {
  display: grid;
  grid-template-columns: 0.5fr 0.5fr;
  grid-template-rows: 0.5fr 0.5fr;
  grid-template-areas:
    "scenarios background"
    "scenarios scenario";
  justify-items: stretch;
  align-items: stretch;
  justify-content: stretch;
  align-content: stretch;
}
</style>
