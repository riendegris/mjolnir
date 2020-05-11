<template>
  <tr class="hover:bg-gray-100 border-b border-gray-200 py-10">
    <td class="px-4 py-4">{{ id }}</td>
    <td class="px-4 py-4">{{ filename }}</td>
    <td class="px-4 py-4">{{ filestatus }}</td>
    <td class="px-4 py-4">
      <div class="flex justify-end items-end p-5 pl-8 pr-8 select-none">
        <button @click.prevent="submitDownload" class="w-7 h-7 flex items-center justify-center px-4">
          <!-- icon by feathericons.com -->
          <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-download">
            <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
            <polyline points="7 10 12 15 17 10"></polyline>
            <line x1="12" y1="15" x2="12" y2="3"></line>
          </svg>
        </button>
        <button @click.prevent="submitRemove" class="w-7 h-7 flex items-center justify-center">
          <!-- icon by feathericons.com -->
          <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-trash-2">
            <polyline points="3 6 5 6 21 6"></polyline>
            <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"></path>
            <line x1="10" y1="11" x2="10" y2="17"></line><line x1="14" y1="11" x2="14" y2="17"></line>
          </svg>
        </button>
      </div>
    </td>
  </tr>
</template>

<script>
import { mapActions } from 'vuex'

export default {
  name: 'banoitem',
  props: [
    'bano',
    'id',
    'filename',
    'filestatus'
  ],
  methods: {
    ...mapActions({
      removeBanoItem: 'banos/removeBanoItem',
      downloadBanoItem: 'banos/downloadBanoItem'
    }),
    submitRemove () {
      this.removeBanoItem({ id: this.bano, iid: this.id })
    },
    submitDownload () {
      this.downloadBanoItem({ id: this.bano, iid: this.id })
    }
  }
}
</script>
