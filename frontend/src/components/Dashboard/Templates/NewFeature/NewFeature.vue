<template>
  <div class="flex m-6 items-center justify-center bg-blue-100">
    <div class="flex flex-col w-full">
      <h3 class="bg-blue-800 text-gray-300 text-center p-6">Load a feature from file</h3>
      <file-reader @load="text = $event" class="mt-6 mx-6" />
      <textarea rows="10" v-model="text" class="m-6 bg-blue-100 border border-blue-800"></textarea>
      <div class="mb-6 mx-6">
        <button @click.prevent="upload" class="w-64 flex flex-col items-center px-2 py-3 bg-white text-gray-800 rounded-lg shadow-lg tracking-wide uppercase border border-blue cursor-pointer hover:bg-blue hover:text-black">
          <span class="mt-2 text-base leading-normal">Upload</span>
        </button>
      </div>
    </div>
  </div>
</template>

<script>
import FileReader from '@/components/FileReader'
import { mapActions } from 'vuex'

export default {
  name: 'NewFeature',
  data: () => ({ text: '' }),
  components: {
    FileReader
  },
  methods: {
    ...mapActions({
      uploadFeature: 'features/uploadFeature'
    }),
    upload () {
      const { text } = this
      this.uploadFeature({ text })
    }
  }
}
</script>
