<template>
    <main class="main bg-blue-600">
    <MainOverview />
    <MainHeader />
    <MainPanel :key='panel' :panel='panel'/>
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
      panel: 'dashboard/panel'
    })
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
