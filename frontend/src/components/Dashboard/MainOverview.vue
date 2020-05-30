<template>
  <div class="main-overview m-6 gap-6">
    <div class="overviewcard bg-blue-800 text-gray-500">
      <div class="overviewcard__icon">Overview</div>
      <div class="overviewcard__info">Card</div>
    </div>
    <div class="overviewcard bg-blue-800 text-gray-500">
      <div class="overviewcard__icon">Overview</div>
      <div class="overviewcard__info">Card</div>
    </div>
    <div class="overviewcard bg-blue-800 text-gray-500">
      <div class="overviewcard__icon">Overview</div>
      <div class="overviewcard__info">Card</div>
    </div>
    <div class="overviewcard bg-blue-800 text-gray-500">
      <div class="overviewcard__icon">Overview</div>
      <div class="overviewcard__info">Card</div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'MainOverview'
}
</script>

<style>
  .main-overview {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(265px, 1fr)); /* Where the magic happens */
    grid-auto-rows: 94px;
  }
  .overviewcard {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 20px;
    background-color: #d3d3;
  }
</style>
