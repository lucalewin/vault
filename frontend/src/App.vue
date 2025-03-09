<template>
  <Header />
  <main id="main">
    <router-view></router-view>
  </main>
</template>

<script setup lang="ts">
import Header from './components/Header.vue';
</script>

<style>
html,
body {
  margin: 0;
  padding: 0;
  width: 100%;
  height: 100%;
  background-color: #121212;
  color: #e0e0e0;
}

#app {
  display: flex;
  flex-direction: column;
  justify-content: flex-start;
  height: 100%;
  width: 100%;
}

#main {
  height: 100%;
  margin: 0 auto;
  max-width: 600px;
  width: 100%;
  padding-bottom: 1rem;
  background-color: #121212;
  color: #e0e0e0;
  /* Remove the border above the password list container */
}
</style>