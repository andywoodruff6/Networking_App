<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
//import Sidebar from './components/sidebar/Sidebar.vue' 
</script>

<template>
  <!-- <Sidebar /> -->
  <div class="container">
    <h1>Welcome to Tauri!</h1>

    <router-link to="/">Home</router-link>
    <router-link to="/another">Another Page</router-link>

    <router-view />

  </div>
</template>

<style scoped>

</style>
