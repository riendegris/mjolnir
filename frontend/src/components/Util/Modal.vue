<!-- Based on code from https://alligator.io/vuejs/vue-modal-component/ -->

<template>
  <transition name="modal-fade">
    <div class="fixed inset-0 flex justify-center items-center" style="background-color: rgba(0, 0, 0, 0.3);">
      <div class="p-2 bg-white overflow-x-auto flex flex-col rounded-lg">
        <button @click="close" class="w-7 h-7 flex justify-end">
          <!-- icon by feathericons.com -->
          <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#666666" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="feather feather-x-circle"><circle cx="12" cy="12" r="10"></circle><line x1="15" y1="9" x2="9" y2="15"></line><line x1="9" y1="9" x2="15" y2="15"></line></svg>
        </button>
        <header class="px-4 pb-4 flex justify-between">
          <slot name="header" class="mx-4">
            This is the default tile!
          </slot>
        </header>
        <section class="relative">
          <slot name="body">
            I'm the default body!
          </slot>
        </section>
        <footer class="pb-2 pr-2 flex justify-end">
          <slot name="footer">
            I'm the default footer!
            <button type="button" class="btn-green" @click="close" >
              Close me!
            </button>
          </slot>
        </footer>
      </div>
    </div>
  </transition>
</template>

<script>
export default {
  name: 'modal',

  methods: {
    close () {
      this.$emit('close')
    }
  }
}
</script>

<style>
 .modal-fade-enter,
  .modal-fade-leave-active {
    opacity: 0;
  }

  .modal-fade-enter-active,
  .modal-fade-leave-active {
    transition: opacity .5s ease
  }
</style>
