<template>
  <div class="index flex flex-col ml-3 mt-3">
    <div class="w-full flex justify-between bg-blue-800 p-6 text-gray-300">
      <span class="uppercase">Index</span>
      <span class="ml-6">{{ truncate(index.signature, 50, true) }}</span>
    </div>
    <div class="flex-grow flex flex-col items-center justify-start bg-blue-100">
      <div class="m-6">
        <p class="italic font-header text-gray-800">{{ index.indexType }} / {{ index.dataSource }} / {{ index.regions }}</p>
      </div>
      <div class="m-6">
        <p class="italic font-header text-gray-800">{{ index.status }}</p>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'Index',
  props: ['index']
}
</script>

<style>
.index {
  grid-area: index;
}
</style>
