<template>
  <div class="indexes flex flex-col mr-3">
    <div class="w-full flex justify-between bg-blue-800 p-6 text-gray-300">
      <span class="uppercase">Environment</span>
      <span class="ml-6">{{ truncate(environment.signature, 30, true) }}</span>
    </div>
    <div class="flex-grow flex flex-col items-center justify-start bg-blue-100">
      <ul class="m-6">
        <li v-for="(index, i) in indexes" :key="index.id" class="font-header">
           <button @click='selectIndex(i)' :class="{'selected': i == idx}" class="flex items-center justify-center my-2 p-4">
             {{ i }} - {{ index.indexType }} - {{ index.dataSource }}
           </button>
        </li>
      </ul>
    </div>
  </div>
</template>

<script>

export default {
  name: 'Indexes',
  props: ['environment', 'indexes'],
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
  @apply bg-red-800;
}
.indexes {
  grid-area: indexes;
}
</style>
