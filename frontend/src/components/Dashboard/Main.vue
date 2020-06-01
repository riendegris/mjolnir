<template>
  <main class="main flex flex-col bg-blue-600">
    <MainOverview />
    <MainHeader />
    <MainPanel :key='sig' :panel='panel' />
  </main>
</template>

<script>
import MainHeader from './MainHeader'
import MainOverview from './MainOverview'
import MainPanel from './MainPanel'
import { mapGetters } from 'vuex'

export default {
  name: 'Main',
  computed: {
    ...mapGetters({
      panel: 'dashboard/panel',
      key: 'dashboard/key',
      value: 'dashboard/value'
    }),
    sig () {
      const s = this.panel + '/' + this.key + '/' + this.value
      console.log('sig: ' + s)
      return s
    }
  },
  components: {
    MainHeader,
    MainOverview,
    MainPanel
  }
}
</script>

<style>
.main {
  grid-area: main;
}

</style>
