<script>
    import { ref } from "vue";
    import { invoke } from "@tauri-apps/api/tauri";

    export default {
     setup() {
            async function createPerson() {
            // Call the create person function here
            }

            async function readPerson() {
              
            }
            async function updatePerson() {
            // Call the update person function here
            }
            async function  deletePerson() {
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
