<template>
  <div v-if="foo" class="flex-grow flex main-cards">
    <Scenarios class="left" :scenarios='scenarios' :feature='feature' @selectIndex='selectScenario'/>
    <!--<Scenario class="right" :steps='steps' :scenario='scenario' />-->
    <Background class="right" :steps='steps' />
  </div>
  <div v-else>
    <p>Loading</p>
  </div>
</template>

<script>
import { mapGetters, mapActions } from 'vuex'
import Scenarios from './Scenarios'
import Background from './Background'

export default {
  name: 'Feature',
  components: {
    Scenarios,
    Background
  },
  data () {
    return {
      idx: 0, // index of the scenario selected by the user, drives the Scenario component
      // it is initialized to 0 here, but it is driven by the Scenarios component.
      foo: false
    }
  },
  computed: {
    ...mapGetters({
      id: 'dashboard/value',
      backgroundLoading: 'features/backgroundLoading',
      scenariosLoading: 'features/scenariosLoading'
      // scenarioLoading: 'features/scenarioLoading',
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
    steps () {
      return this.$store.getters['features/background'](this.id).steps
    },
    hasBackground () {
      // FIXME Are all these tests required
      // To have a background, we must be done with getting the response from the server,
      // and that response must not be null (Some features don't have a background)
      return !this.backgroundLoading && this.background && this.background !== 'null' && this.background !== 'undefined' && this.steps !== 'undefined'
    }
  },
  methods: {
    ...mapActions({
      loadScenarios: 'features/loadScenarios',
      loadBackground: 'features/loadBackground'
    }),
    selectScenario (idx) {
      this.idx = idx
      // const id = this.scenario.id
      // console.log('reloading steps with id ' + id)
      // this.loadSteps({ id })
    }
  },
  async created () {
    const { id } = this
    this.loadScenarios({ id })
    this.loadBackground({ id }).then( () => { this.foo = true })
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
