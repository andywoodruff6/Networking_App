<script>
import { getPeople } from '../services/database.js'
import TopLevelPerson from '../components/TopLevelPerson.vue'

export default {
  name: "NetworkPage",
  components: {
    TopLevelPerson,
  },
  data() {
    return {
      people: [],
    };
  },
  mounted() {
    this.fetchPeople();
  },
  methods: {
    async fetchPeople() {
      try {
        this.people = await getPeople();
      } catch (error) {
        console.error(error);
      }
    },
  },

};
</script>

<template>
  <div class="relationshipPage">
    <div class="display-name">
      <h2>Network</h2>
      <div v-for="person in people" :key="person.id">
        <TopLevelPerson 
          :first_name="person.first_name" 
          :last_name="person.last_name" 
          :id="person.id"
          :relationship="person.relationship" 
          :phone_number="person.phone_number" 
          :email="person.email" 
        />
      </div>
    </div>
    <!-- <div>
      Hello
    </div> -->
  </div>
</template>

<style>

</style>
