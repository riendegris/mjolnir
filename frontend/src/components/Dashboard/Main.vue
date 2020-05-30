<template>
    <main class="main bg-blue-600">
    <MainOverview />
    <MainHeader />
    <MainPanel />
  </main>
</template>

<script>
import MainHeader from './MainHeader'
import MainOverview from './MainOverview'
import MainPanel from './MainPanel'

export default {
  name: 'Main',
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
