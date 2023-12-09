<template>
  <div class="addPersonPage">
    <ul>
      <li v-for="person in people" :key="person.id">{{ person.first_name }}</li>
    </ul>
  </div>
</template>

<script>
import { getPeople } from '../services/database.js'

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
        this.people = await getPeople();
      } catch (error) {
        console.error(error);
      }
    },
  },
};
</script>
