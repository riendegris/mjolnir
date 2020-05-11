<template>
  <div class="h-screen bg-blue-800">
    <div class="w-3/5 p-8 mx-auto bg-gray-100">
      <h1 class="mb-4 text-center">
        BANO Environments <TestCounter />
      </h1>
      <div class="shadow mt-8">
        <Bano v-for="bano in banos" :id='bano.id' :description='bano.description' :items='bano.items' :key='bano.id' class="border-b"/>
      </div>
      <div class="flex justify-end items-end p-5 pl-8 pr-8 select-none">
        <button @click.prevent="modal = !modal" class="rounded-full border border-gray-500 w-7 h-7 flex items-center justify-center">
          <!-- icon by feathericons.com -->
          <svg aria-hidden="true" class="" fill="none" height="24" stroke="#606F7B" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" viewbox="0 0 24 24" width="24" xmlns="http://www.w3.org/2000/svg">
            <line x1="12" y1="8" x2="12" y2="16"></line><line x1="8" y1="12" x2="16" y2="12"></line>
          </svg>
        </button>
      </div>
      <Modal v-show="modal" @close="modal = !modal">
        <template v-slot:header>
          Add a new BANO Environment
        </template>
        <template v-slot:body>
          <div class="bg-white p-4 flex flex-col">
            <div class="-mx-3 md:flex mb-6">
              <div class="md:w-1/2 px-3 mb-6 md:mb-0">
                <label class="block uppercase tracking-wide text-grey-darker text-xs font-bold mb-2" for="grid-id">
                  ID
                </label>
                <input v-model="id"
                  class="appearance-none block w-full bg-grey-lighter text-grey-darker border border-red rounded py-3 px-4 mb-3"
                  id="grid-id" type="text" placeholder="Id">
              </div>
            </div>
            <div class="-mx-3 md:flex mb-6">
              <div class="md:w-full px-3">
                <label class="block uppercase tracking-wide text-grey-darker text-xs font-bold mb-2" for="grid-description">
                  Description
                </label>
                <input v-model="description"
                  class="appearance-none block w-full bg-grey-lighter text-grey-darker border border-grey-lighter rounded py-3 px-4 mb-3"
                  id="grid-description" type="text">
              </div>
            </div>
          </div>
        </template>
        <template v-slot:footer>
          <button @click.prevent="submitForm" class="bg-transparent hover:bg-gray-500 text-gray-700 font-semibold hover:text-white py-2 px-4 border border-gray-500 hover:border-transparent rounded">
            Add
          </button>
        </template>
      </Modal>
    </div>
  </div>
</template>

<script>
import { mapGetters, mapActions } from 'vuex'
import Bano from './Bano'
import TestCounter from '@/components/Util/TestCounter'
import Modal from '@/components/Util/Modal'

export default {
  name: 'banos',
  data () {
    return {
      modal: false, /* this tells whether the modal window is visible or not */
      id: null, /* this is an bound to the modal */
      description: null /* this is an bound to the modal */
    }
  },
  components: {
    Bano,
    TestCounter,
    Modal
  },
  computed: {
    ...mapGetters({
      banos: 'banos/banos'
    })
  },
  methods: {
    ...mapActions({
      loadBanos: 'banos/loadBanos',
      addBano: 'banos/addBano'
    }),
    showNotification (ev) {
      this.$store.dispatch('notifications/addNotification', { title: 'Error in Foo', message: 'you should pay attention', theme: 'error', timeout: 3000 })
    },
    submitForm () {
      const { id, description } = this
      this.addBano({ id, description })
      this.modal = !this.modal
    }
  },
  async created () {
    this.loadBanos()
  }
}
</script>

<style>
  /* Tab content - closed */
  .tab-content {
    max-height: 0;
    -webkit-transition: max-height .35s;
    -o-transition: max-height .35s;
    transition: max-height .35s;
  }
  /* :checked - resize to full height */
  .tab input:checked ~ .tab-content {
    max-height: 100vh;
  }
</style>
