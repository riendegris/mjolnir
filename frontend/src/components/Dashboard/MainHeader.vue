<template>
  <div class="main-header flex justify-between m-6 p-6 bg-blue-100 text-gray-600">
    <div class="main-header__heading">Hello User</div>
    <div class="main-header__updates">Recent Items</div>
  </div>
</template>

<script>
export default {
  name: 'MainHeader'
}
</script>

<style>
  .main-header {
    height: 150px; /* Force our height since we don't have actual content yet */
  }
</style>
