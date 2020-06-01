<template>
 <div class="loader" :class="{ 'visible': visible }" >
   Loading
  </div>
</template>

<script>
export default {
  name: 'loader',
  props: ['visible']
}
</script>

<style>
.loader {
  background: rgba(255, 255, 255, 0.8);
  transition: 0.15s ease-in opacity;
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  pointer-events: none;
  opacity: 0;
}
.visible {
  opacity: 1;
  pointer-events: all;
}
</style>
