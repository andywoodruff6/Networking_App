<script>
import { getPeopleByRelationship } from '../services/database.js'
import TopLevelPerson from '../components/TopLevelPerson.vue'

export default {
  name: "FriendPage",
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
        this.people = await getPeopleByRelationship("friend");
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
      <h2>Friends</h2>
      <div v-for="person in people" :key="person.id">
        <TopLevelPerson
          :first_name="person.first_name"
          :last_name="person.last_name"
          :id="person.id"
          />
      </div>
    </div>
  </div>
</template>

<style></style>