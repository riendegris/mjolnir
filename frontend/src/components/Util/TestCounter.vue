<template>
  <p>{{ counter }}</p>
</template>

<script>
import { mapGetters } from 'vuex'

export default {
  name: 'testcounter',
  computed: {
    ...mapGetters({
      counter: 'test/counter'
    })
  }
}
</script>
