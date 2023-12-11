<template>
  <div>
    <h1>homePage</h1>
    <AddOne />

    <!-- <CrudCheck />  -->
  </div>
</template>

<script >
//import CrudCheck from "../components/CrudCheck.vue";
import AddOne from "../components/AddOne.vue";
export default {
  name: 'homePage',
  components: {
    //CrudCheck,
    AddOne
  },
}
</script>

<style scoped>
/* Add your custom styles here */
</style>
