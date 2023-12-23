<script>

export default {
  name: 'Sidebar',
}
</script>

<template>
  <div class="sidebar">
    <router-link to="/" class="icon-link" active-class="active">
      <i class="fa-solid fa-house fa-2xl" />
    </router-link>
    <router-link to="/calendar" class="icon-link" active-class="active">
      <i class="fas fa-calendar-days fa-2xl" />
    </router-link>
    <router-link to="/network" class="icon-link" active-class="active">
      <i class="fa-solid fa-circle-nodes fa-2xl" />
    </router-link>
    <router-link to="/friend" class="icon-link" active-class="active">
      <i class="fa-solid fa-user-group fa-2xl" />
    </router-link>
    <router-link to="/work" class="icon-link" active-class="active">
      <i class="fa-solid fa-briefcase fa-2xl" />
    </router-link>
    <router-link to="/hobby" class="icon-link" active-class="active">
      <i class="fa-solid fa-user-astronaut fa-2xl" />
    </router-link>
    <router-link to="/addPerson" class="icon-link" active-class="active">
      <i class="fa-solid fa-user-plus fa-2xl" />
    </router-link>
  </div>
</template>

<style scoped>
.sidebar {
  min-width: 60px;
}
</style>
