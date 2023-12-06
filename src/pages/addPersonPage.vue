<template >
  <div class="addPersonPage">
    <h1>addPerson</h1>

    <div class="input-block">
      <label for="name">First Name</label>
      <input type="text" id="first_name" />

      <label for="name">Last Name</label>
      <input type="text" id="last_name" />
    </div>
  </div>
</template>

<script>
export default {
  // Your script code here
}
</script>

<style scoped>
.input-block {
  display: flex;
  flex-direction: column;
  align-items: left;
  max-width: 300px;
  padding: 20px;
  margin: 20px;

  border: 1px solid red;
}

.addPersonPage {
  border: 1px solid blue;
  order: 2;
  margin-left: 100px;
}
</style>
