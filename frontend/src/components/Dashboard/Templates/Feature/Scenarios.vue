<template>
  <div class="scenarios flex flex-col mr-3">
    <div class="w-full flex justify-between bg-blue-800 p-6 text-gray-300">
      <span class="uppercase">Feature</span>
      <span class="ml-6">{{ truncate(feature.name, 50, true) }}</span>
    </div>
    <div class="flex-grow flex flex-col items-center justify-start bg-blue-100">
      <div class="m-6">
        <p class="italic font-header text-gray-800">{{ feature.description }}</p>
      </div>
      <div class="m-6">
        <p class="italic font-header text-gray-800">{{ feature.tags }}</p>
      </div>
      <ul>
        <li v-for="(scenario, index) in scenarios" :key="scenario.id" class="font-header">
           <button @click='selectIndex(index)' :class="{'selected': index == idx}" class="flex items-center justify-center my-2 p-4">
             {{ index }} - {{ truncate(scenario.name, 50, true) }}
           </button>
        </li>
      </ul>
    </div>
  </div>
</template>

<script>

export default {
  name: 'Scenarios',
  props: ['feature', 'scenarios'],
  data () {
    return {
      idx: 0
    }
  },
  methods: {
    selectIndex (event) {
      console.log('select Index: ' + event)
      this.idx = event
      this.$emit('selectIndex', event)
    }
  }
}
</script>

<style>
.selected {
  @apply bg-gray-300 border border-gray-800
}
.scenarios {
  grid-area: scenarios;
}
</style>
