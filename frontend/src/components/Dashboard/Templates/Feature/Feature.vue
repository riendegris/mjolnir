<template>
  <div v-if="foo" class="flex-grow flex main-cards">
    <Scenarios :scenarios='scenarios' :feature='feature' @selectIndex='selectScenario'/>
    <Scenario :scenario='scenario' />
    <Background :background='background' />
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
      this.scenario = this.$store.getters['features/scenarios'](this.id)[this.idx]
      await this.loadScenarioSteps({feature: this.id, scenario: this.scenario.id})
      this.scenario = this.$store.getters['features/scenarios'](this.id)[this.idx]
    }
  },
  async created () {
    await this.loadScenarios({ id: this.id })
    this.scenario = this.$store.getters['features/scenarios'](this.id)[this.idx]
    await this.loadBackground({ id: this.id })
    console.log('scenario id: ' + this.scenario.id)
    await this.loadScenarioSteps({feature: this.id, scenario: this.scenario.id})
    this.scenario = this.$store.getters['features/scenarios'](this.id)[this.idx]
    this.foo = true
  }
}
</script>

<style>
.main-cards {
  display: grid;
  grid-template-columns: 0.5fr 0.5fr;
  grid-template-rows: 0.5fr 0.5fr;
  grid-template-areas:
    "scenarios background"
    "scenarios scenario";
}
</style>
