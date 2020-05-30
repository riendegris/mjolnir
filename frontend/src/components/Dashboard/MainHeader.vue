<template>
  <div class="main-header">
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
    display: flex;
    justify-content: space-between;
    margin: 20px;
    padding: 20px;
    height: 150px; /* Force our height since we don't have actual content yet */
    background-color: #e3e4e6;
    color: slategray;
  }
</style>
