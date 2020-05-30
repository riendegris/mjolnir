<template>
  <div class="grid-container">
    <Header />
    <SideNav />
    <Main />
    <Footer />
    <Logo />
  </div>
</template>

<script>
import Header from './Header'
import Footer from './Footer'
import SideNav from './SideNav'
import Main from './Main'
import Logo from './Logo'

export default {
  name: 'Home',
  components: {
    Header,
    Footer,
    SideNav,
    Main,
    Logo
  }
}
</script>

<style>
/* Simple dashboard grid CSS */

/* Assign grid instructions to our parent grid container */
.grid-container {
  display: grid;
  grid-template-columns: 240px 1fr;
  grid-template-rows: 0.075fr 0.85fr 0.075fr;
  grid-template-areas:
    "logo header"
    "sidenav main"
    "sidenav footer";
  height: 100vh;
}
</style>
