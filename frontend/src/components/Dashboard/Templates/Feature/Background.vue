<template>
  <div class="background flex flex-col ml-3 mb-3">
    <div class="w-full flex justify-between bg-blue-800 p-6 text-gray-300">
      <span class="uppercase">Background</span>
    </div>
    <div class="flex flex-col items-center justify-start bg-blue-100">
      <ul class="my-6">
        <li v-for="step in background.steps" :key="step.id">
          <span class="font-semibold">{{ step.stepType }} </span><span>{{ step.value }}</span>
        </li>
      </ul>
    </div>
    <div>
      <button @click.prevent='switchPanel({"panel": "Environment", "key": "id", "value": background.environment.id })' class="w-64 flex flex-col items-center px-2 py-3 bg-gray-300 text-gray-800 rounded-lg shadow-lg tracking-wide uppercase border border-blue cursor-pointer hover:bg-blue hover:text-black">
        <span class="mt-2 text-base leading-normal">Environment</span>
      </button>
    </div>
  </div>
</template>

<script>
import { mapActions } from 'vuex'

export default {
  name: 'Background',
  props: ['background'],
  methods: {
    ...mapActions({
      switchPanel: 'dashboard/switchPanel'
    })
  }
}
</script>

<style>
.background {
  grid-area: background;
}
</style>
