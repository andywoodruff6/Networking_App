<template >
  <div class="addPersonPage">
    <h1>addPerson</h1>

    <div class="input-block">
      <div class="input-card">
        <label for="first_name">First Name</label>
        <input type="text" id="first_name" v-model="first_name" />
      </div>
      <div class="input-card">
        <label for="last_name">Last Name</label>
        <input type="text" id="last_name" v-model="last_name" />
      </div>
      <div class="input-card">
        <label for="relationship">Relationship</label>
        <select id="relationship" v-model="relationship">
          <option value="">Select...</option>
          <option value="friend">Friend</option>
          <option value="work">Work</option>
          <option value="hobby">Hobby</option>
        </select>
      </div>
      <div class="input-card">
        <label for="email">Email</label>
        <input type="text" id="email" v-model="email" />
      </div>
      <div class="input-card">
        <label for="phone_number">Phone Number</label>
        <input type="text" id="phone_number" v-model="phone_number" />
      </div>
      <div>
        <button class="button" @click.prevent="submitForm">Add Person</button>
      </div>
    </div>
  </div>
</template>

<script>
import Database from 'tauri-plugin-sql-api';

export default {
  data() {
    return {
      first_name: '',
      last_name: '',
      relationship: '',
      email: '',
      phone_number: ''
    }
  },
  methods: {
    async submitForm() {
      try {
        // refactor this into service/database.js
        const db = await Database.load("sqlite:test.db");
        await db.execute(
          'INSERT INTO person (first_name, last_name, relationship, email, phone_number) VALUES (?,?,?,?, ?)',
          [this.first_name, this.last_name, this.relationship, this.email, this.phone_number])
        console.log('Person added');
      } catch (error) {
        console.error(error);
      }
    },
  },
}
</script>

<style scoped>
.input-card {
  display: flex;
  flex-direction: column;
  margin: 0 0 1rem 0;
}
</style>
