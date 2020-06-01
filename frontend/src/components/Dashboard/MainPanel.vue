<template>
  <div class="main-panel flex-grow m-6 flex flex-col">
    <component :is="componentLoader"/>
  </div>
</template>

<script>
export default {
  name: 'MainPanel',
  props: ['panel'],
  computed: {
    componentLoader () {
      return () => import('./Templates/' + this.panel + '/' + this.panel)
      // For debugging panel styling, it's easier to hardcode them
      // return () => import('./Templates/Feature/Feature')
    }
  }
}
</script>
