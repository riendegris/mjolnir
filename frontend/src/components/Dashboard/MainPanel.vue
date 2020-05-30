<template>
  <div class="main-panel">
    <div class="card">Card1</div>
    <div class="card">Card2</div>
    <div class="card">Card3</div>
  </div>
</template>

<script>
export default {
  name: 'MainPanel'
}
</script>

<style>
  .main-panel {
    column-count: 2;
    column-gap: 20px;
    margin: 20px;
    height: 100%;
  }
  .card {
    display: flex;
    flex-direction: column;
    align-items: center;
    width: 100%;
    background-color: #82bef6;
    margin-bottom: 20px;
    -webkit-column-break-inside: avoid;
    padding: 24px;
    box-sizing: border-box;
  }

  /* Force varying heights to simulate dynamic content */
  .card:first-child {
    background: red;
  }

  .card:nth-child(2) {
  }

  .card:nth-child(3) {
  }

</style>
