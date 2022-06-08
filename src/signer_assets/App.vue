<template>
  <div id="nav">
    <router-link to="/"> IC-Signer </router-link>
    <!-- <router-link to="/"> Home </router-link> | -->
    <!-- <router-link to="/tools"> Tools </router-link> -->
  </div>
  <router-view/>
</template>

<style lang="less">
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  font-size: 125%;
  color: #2c3e50;
}

#nav {
  padding: 30px;

  a {
    font-weight: bold;
    color: #000000;

    &.router-link-exact-active {
      color: #409EFF;
    }
  }
}
</style>
