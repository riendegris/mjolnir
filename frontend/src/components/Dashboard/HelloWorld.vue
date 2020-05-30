<template>
  <div class="hello">
    <h1 class='font-header text-lg'>{{ msg }}</h1>
    <p class='font-text'>
      We're ready to rock'n roll
    </p>
  </div>
</template>

<script>
export default {
  name: 'HelloWorld',
  props: {
    msg: String
  }
}
</script>
