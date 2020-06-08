<template>
  <div v-if="foo" class="environment-panel flex-grow">
    <Indexes :indexes='indexes' :environment='environment' @selectIndex='selectIndex'/>
    <Index :index='index' />
  </div>
  <div v-else>
    <p>Loading</p>
  </div>
</template>

<script>
import { mapGetters, mapActions } from 'vuex'
import Indexes from './Indexes'
import Index from './Index'

export default {
  name: 'Environment',
  components: {
    Indexes,
    Index
  },
  data () {
    return {
      idx: 0, // index of the index selected by the user, drives the Index component
      // it is initialized to 0 here, but it is driven by the Indexes component.
      // FIXME Get a more descriptive variable name
      foo: false,
      index: {}
    }
  },
  computed: {
    ...mapGetters({
      id: 'dashboard/value'
    }),
    environment () {
      return this.$store.getters['environments/environment'](this.id)
    },
    indexes () {
      return this.$store.getters['environments/indexes'](this.id)
    }
  },
  methods: {
    ...mapActions({
      loadIndexes: 'environments/loadIndexes'
    }),
    async selectIndex (idx) {
      this.idx = idx
      this.index = this.$store.getters['environments/indexes'](this.id)[this.idx]
    }
  },
  async created () {
    await this.loadIndexes({ id: this.id })
    this.index = this.$store.getters['environments/indexes'](this.id)[this.idx]
    this.foo = true
  }
}
</script>

<style>
.environment-panel {
  display: grid;
  grid-template-columns: 0.5fr 0.5fr;
  grid-template-rows: 0.5fr 0.5fr;
  grid-template-areas:
    "indexes index"
    "indexes index";
  justify-items: stretch;
  align-items: stretch;
  justify-content: stretch;
  align-content: stretch;
}
</style>
