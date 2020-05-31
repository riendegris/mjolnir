<template>
  <label class="w-64 flex flex-col items-center px-2 py-3 bg-gray-light hover:bg-gray rounded-lg shadow-lg tracking-wide uppercase border border-blue cursor-pointer text-gray-800 hover:text-gray">
    <span class="mt-2 text-base leading-normal">Select a file</span>
    <input type='file' class="hidden" @change="loadTextFromFile"/>
  </label>
</template>

<script>
export default {
  methods: {
    loadTextFromFile (ev) {
      const file = ev.target.files[0]
      const reader = new FileReader()

      reader.onload = e => this.$emit('load', e.target.result)
      reader.readAsText(file)
    }
  }
}
</script>
