<template>
  <div class="scenario flex flex-col ml-3 mt-3">
    <div class="w-full flex justify-between bg-blue-800 p-6 text-gray-300">
      <span class="uppercase">Scenario</span>
      <span class="ml-6">{{ truncate(scenario.name, 50, true) }}</span>
    </div>
    <div class="flex-grow flex flex-col items-center justify-start bg-blue-100">
      <div class="m-6">
        <p class="italic font-header text-gray-800">{{ scenario.tags }}</p>
      </div>
      <ul class="my-6">
        <li v-for="step in scenario.steps" :key="step.id">
          <span class="font-semibold">{{ step.stepType }} </span><span>{{ step.value }}</span>
        </li>
      </ul>
    </div>
  </div>
</template>

<script>
export default {
  name: 'Scenario',
  props: ['scenario']
}
</script>

<style>
.scenario {
  grid-area: scenario;
}
</style>
