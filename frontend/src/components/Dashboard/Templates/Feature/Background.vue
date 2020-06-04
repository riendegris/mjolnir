<template>
  <div class="background flex flex-col ml-3 mb-3">
    <div class="w-full flex justify-between bg-blue-800 p-6 text-gray-300">
      <span class="uppercase">Background</span>
    </div>
    <div class="flex-grow flex flex-col items-center justify-start bg-blue-100">
      <ul class="my-6">
        <li v-for="step in background.steps" :key="step.id">
          <span class="font-semibold">{{ step.stepType }} </span><span>{{ step.value }}</span>
        </li>
      </ul>
    </div>
  </div>
</template>

<script>
export default {
  name: 'Background',
  props: ['background']
}
</script>

<style>
.background {
  grid-area: background;
}
</style>
