<script>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

export default {
  setup() {
    const number = ref(0);

    async function addOne() {
      number.value = await invoke("add_one", { number: number.value });
    }

    return { number, addOne };
  }
}
</script>

<template>
    <div class="AddOne">
      <p>Current value: {{ number }}</p>
      <button @click="addOne">Add 1</button>
    </div>
</template>

<style scoped>
.AddOne:border {
  border: 10px solid red;
}


</style>