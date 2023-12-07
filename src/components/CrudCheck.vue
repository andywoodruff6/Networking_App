<script>
import { ref, onMounted } from "vue";
// import { invoke } from "@tauri-apps/api/tauri";
import Database from "tauri-plugin-sql-api";
import { logError } from "../helper/helper-functions";

export default {
  setup() {
    onMounted(async () => {
      const db = await Database.load("sqlite:test.db");
      try {
        // sqlite. The path is relative to `tauri::api::path::BaseDirectory::App`.
        // const db = await Database.load("sqlite:test.sqlite");
        console.log("test");
        // Create a table.
        await db.execute(
          "CREATE TABLE IF NOT EXISTS person (id INTEGER PRIMARY KEY, name TEXT)"
        );
        console.log("Created table? maybe");
      } catch (err) {
        logError(err);
      }

      await db.execute("INSERT INTO person (id, name) VALUES (?, ?)", [1, "John Doe"]);
      console.log("Inserted a person");

      let result = await db.select(
        "SELECT * FROM person",
      );
      console.log("Read a person", result);
    });

    async function createPerson() {
      // Call the create person function here
    }
    async function readPerson() {

    }
    async function updatePerson() {
      // Call the update person function here
    }
    async function deletePerson() {
      // Call the delete person function here
    }
    return { createPerson, readPerson, updatePerson, deletePerson };
  }
}
</script>

<template>
  <div>
    <button @click="createPerson">Create Person</button>
    <button @click="readPerson">Read Person</button>
    <button @click="updatePerson">Update Person</button>
    <button @click="deletePerson">Delete Person</button>
  </div>
</template>

<style>
/* Add any necessary styles here */
</style>
