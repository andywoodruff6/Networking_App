<script>
import { getPeopleByRelationship } from '../services/database.js'
import TopLevelPerson from '../components/TopLevelPerson.vue'

export default {
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
        this.people = await getPeopleByRelationship("hobby");
      } catch (error) {
        console.error(error);
      }
    },
  },
  components: {
    TopLevelPerson,
  },
};
</script>

<template>
  <div class="addPersonPage">
    <div v-for="person in people" :key="person.id">
      <TopLevelPerson :first_name="person.first_name" :last_name="person.last_name" :id="person.id" />
    </div>
  </div>
</template>

<style></style>