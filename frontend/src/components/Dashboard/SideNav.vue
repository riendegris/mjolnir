<template>
  <aside class="sidenav bg-blue-800 flex flex-col">
    <ul class="sidenav__list font-header p-0 mt-16 list-none">
      <li class="sidenav__list-item">Item One</li>
      <li class="sidenav__list-item">Item Two</li>
      <li class="sidenav__list-item">Item Three</li>
      <li class="sidenav__list-item">Item Four</li>
      <li class="sidenav__list-item">Item Five</li>
    </ul>
  </aside>
</template>

<script>

export default {
  name: 'SideNav'
}
</script>

<style>
.sidenav {
  grid-area: sidenav;
}

.sidenav__list-item {
  padding: 20px 20px 20px 40px;
  color: #ddd;
}

.sidenav__list-item:hover {
  background-color: rgba(255, 255, 255, 0.2);
  cursor: pointer;
}
</style>
