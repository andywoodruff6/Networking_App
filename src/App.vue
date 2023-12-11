<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { onMounted } from 'vue'
import Sidebar from './components/sidebar/Sidebar.vue'
import { checkAndCreateTables } from './services/database.js'; 

onMounted(async () => {
  await checkAndCreateTables();
});
</script>

<template>
  <div class="container">
    <Sidebar />
    <router-view />

  </div>
</template>

<style scoped></style>
