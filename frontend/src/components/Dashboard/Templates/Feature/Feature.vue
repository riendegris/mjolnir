<template>
  <div v-if="bar" class="flex-grow flex main-cards">
    <Scenarios class="left" :scenarios='scenarios' :feature='feature' @selectIndex='selectScenario'/>
    <!--<Scenario class="right" :steps='steps' :scenario='scenario' />-->
    <Background class="right" :background='background' />
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
      foo: false,
      bar: false
    }
  },
  computed: {
    ...mapGetters({
      id: 'dashboard/value',
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
      loadBackground: 'features/loadBackground'
    }),
    selectScenario (idx) {
      this.idx = idx
    }
  },
  async created () {
    const { id } = this
    this.loadScenarios({ id })
    this.loadBackground({ id }).then( resp => {
      console.log('load background: ' + resp)
      this.foo = true
      }, error => {
        console.log(error)
      })
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
