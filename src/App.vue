<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { onMounted } from 'vue'
import Sidebar from './components/sidebar/Sidebar.vue'
import AddHistory from './components/addHistory/AddHistory.vue'
import { checkAndCreateTables } from './services/database.js';

onMounted(async () => {
  await checkAndCreateTables();
});
</script>

<template>
  <div class="container">
    <Sidebar />
    <router-view class="main-view"/>

    <AddHistory />
  </div>
</template>


<style scoped>
.container {
  margin: 0;
  padding-top: 2vh;
  text-align: center;
  display: grid;
  grid-template-columns: 120px 1fr 100px;
  grid-template-rows: 1fr;
  grid-column-gap: 10px;
  grid-row-gap: 0px;
}

.sidebar {
  color: var(--tan);
  background-color: var(--dark-green);

  float: left;
  position: fixed;
  top: 0;
  left: 0;
  bottom: 0;
  padding: 0.5em;
  max-width: 150px;

  display: flex;
  flex-direction: column;
}
.main-view {
  grid-column: 2;
  /* border: 1px solid var(--dark-green); */
}

.addHistory {
  position: fixed;
  margin: 20px;
  bottom: 0;
  right: 0;

}
</style>
