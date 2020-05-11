<template>
  <!-- Adapted collapsible accordion from tailwind components: https://tailwindcomponents.com/component/collapsible-accordion -->
  <div class="border-b">
    <div :class="{'border-l-2': active, 'bg-grey-lightest': active, 'border-blue-600': active}">
      <div @click.prevent="active = !active" class="flex justify-between items-center p-5 pl-8 pr-8 cursor-pointer select-none">
        <span class="text-grey-darkest font-thin text-xl">
          {{ description }}
        </span>
        <div v-if="!active" class="rounded-full border bg-grey w-7 h-7 flex items-center justify-center">
          <!-- icon by feathericons.com -->
          <svg aria-hidden="true" class="" fill="none" height="24" stroke="#606F7B" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" viewbox="0 0 24 24" width="24" xmlns="http://www.w3.org/2000/svg">
            <polyline points="6 9 12 15 18 9"></polyline>
          </svg>
        </div>
        <div v-else class="rounded-full border border border-blue-600 w-7 h-7 flex items-center justify-center bg-blue-600">
          <!-- icon by feathericons.com -->
          <svg aria-hidden="true" fill="none" height="24" stroke="white" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" viewbox="0 0 24 24" width="24" xmlns="http://www.w3.org/2000/svg">
            <polyline points="18 15 12 9 6 15"> </polyline>
          </svg>
        </div>
      </div>
      <transition name="foo">
        <div v-show="active">
          <div class="overflow-x-auto mt-2">
            <table class="table-auto border-collapse w-full">
              <thead>
                <tr class="rounded-lg text-sm font-medium text-gray-700 text-left">
                  <th class="px-4 py-2 bg-gray-200">Id</th>
                  <th class="px-4 py-2 bg-gray-200">Filename</th>
                  <th class="px-4 py-2 bg-gray-200">Status</th>
                  <th class="px-4 py-2 bg-gray-200"><!-- actions --></th>
                </tr>
              </thead>
              <tbody class="text-sm font-normal text-gray-700">
                <BanoItem v-for="item in items"
                  :bano='id'
                  :id='item.id'
                  :filename='item.filename'
                  :key='item.id'
                  :filestatus='item.filestatus' />
              </tbody>
            </table>
          </div>
          <div class="flex justify-end items-end p-5 pl-8 pr-8 select-none">
            <p v-show="!items.length" class="text-grey-dark text-sm italic mr-32">(No items)</p>
            <button @click.prevent="modal = !modal" class="rounded-full border border-gray-500 w-7 h-7 flex items-center justify-center">
              <!-- icon by feathericons.com -->
              <svg aria-hidden="true" class="" fill="none" height="24" stroke="#606F7B" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" viewbox="0 0 24 24" width="24" xmlns="http://www.w3.org/2000/svg">
                <line x1="12" y1="8" x2="12" y2="16"></line><line x1="8" y1="12" x2="16" y2="12"></line>
              </svg>
            </button>
          </div>
        </div>
      </transition>
    </div>
    <Modal v-show="modal" @close="modal = !modal">
      <template v-slot:header>
        Add a new BANO Item
      </template>
      <template v-slot:body>
        <div class="bg-white p-4 flex flex-col">
          <div class="-mx-3 md:flex mb-6">
            <div class="md:w-1/2 px-3 mb-6 md:mb-0">
              <label class="block uppercase tracking-wide text-grey-darker text-xs font-bold mb-2" for="grid-id">
                ID
              </label>
              <input v-model="iid"
                class="appearance-none block w-full bg-grey-lighter text-grey-darker border border-red rounded py-3 px-4 mb-3"
                id="grid-id" type="text" placeholder="Id">
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
</template>

<script>
import { mapActions } from 'vuex'
import BanoItem from './BanoItem'
import Modal from '@/components/Util/Modal'

export default {
  name: 'bano',
  components: {
    BanoItem,
    Modal
  },
  data () {
    return {
      active: false, /* This tells whether this item is unfolded in the accordion */
      modal: false, /* This tells if the modal is visible */
      iid: null
    }
  },
  props: [
    'id',
    'description',
    'items'
  ],
  methods: {
    ...mapActions({
      addBanoItem: 'banos/addBanoItem'
    }),
    submitForm () {
      const { id, iid } = this
      this.addBanoItem({ id, iid })
      this.modal = !this.modal
    }
  }
}
</script>

<style>
.foo-enter-active, .fade-leave-active {
  /*  max-height: 0; */
  -webkit-transition: max-height 5s;
  -o-transition: max-height 5s;
  transition: max-height 5s;
}

.foo-enter, .foo-leave {
  max-height: 0;
}
</style>
