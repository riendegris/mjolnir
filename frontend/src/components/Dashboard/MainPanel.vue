<template>
  <div class="main-panel">
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
    }
  }
}
</script>
