<template>
  <div class="flex-grow flex main-cards">
    <Scenarios class="left" :scenarios='scenarios' :feature='feature' @selectIndex='selectScenario'/>
    <Scenario class="right" :steps='steps' :scenario='scenario' />
  </div>
</template>

<script>
import { mapGetters, mapActions } from 'vuex'
import Scenarios from './Scenarios'
import Scenario from './Scenario'

export default {
  name: 'Feature',
  components: {
    Scenarios,
    Scenario
  },
  // For debugging this panel, I hardcode all the details
  // data () {
  //   return {
  //     id: '7789e6c6-4b6e-480f-ae6e-8c561649dcc4',
  //     scenarios: [
  //       { id: 'foo', name: 'select right hand side' },
  //       { id: 'bar', name: 'select left hand side' },
  //       { id: 'baz', name: 'select right foot side' },
  //       { id: 'qux', name: 'select barehanded side' }
  //     ],
  //     feature: {
  //       id: 'foo',
  //       name: 'Baseline Acceptance Test',
  //       description: 'These are for regression testing',
  //       tags: ['regression']
  //     }
  //   }
  // }
  data () {
    return {
      idx: 0 // index of the scenario selected by the user, drives the Scenario component
      // it is initialized to 0 here, but it is driven by the Scenarios component.
    }
  },
  computed: {
    ...mapGetters({
      id: 'dashboard/value',
      scenarios: 'scenarios/scenarios',
      getFeatureById: 'features/feature',
      steps: 'steps/steps'
    }),
    feature () {
      return this.getFeatureById(this.id)
    },
    scenario () {
      return this.scenarios[this.idx]
    }
  },
  watch: {
    // when scenario becomes available, we load new steps
    scenario: function (scenario) {
      if (scenario !== undefined) {
        const sid = scenario.id
        console.log('scenario id ' + sid)
        this.loadSteps({ id: sid })
      }
    }
  },
  methods: {
    ...mapActions({
      loadScenarios: 'scenarios/loadScenarios',
      loadSteps: 'steps/loadSteps'
    }),
    selectScenario (idx) {
      this.idx = idx
      const id = this.scenario.id
      console.log('reloading steps with id ' + id)
      this.loadSteps({ id })
    }
  },
  async created () {
    const id = this.id
    console.log('Creating Feature with id: ' + this.id)
    this.loadScenarios({ id })
  }
}
</script>

<style>
.left {
  flex: 50%;
  margin-right: 0.75rem;
}
.right {
  flex: 50%;
  margin-left: 0.75rem;
}
</style>
