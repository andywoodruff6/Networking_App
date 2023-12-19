<script>
import CalendarCard from '../components/calendar/CalendarCard.vue';
import { testDBCalendar } from '../services/database';

export default {
  name: 'CalendarPage',
  components: {
    CalendarCard,
  },
  data() {
    return {
      calendarArray: [],
    };
  },
  mounted() {
    this.fetchCalendar();
  },
  methods: {
    async fetchCalendar() {
      try {
        this.calendarArray = await testDBCalendar();
      } catch (error) {
        console.error(error);
      }
    },
  }
}
</script>

<template>
  <div>
    <h1>Calendar</h1>
    <div v-for="person in calendarArray" :key="calendarArray.id">
    <CalendarCard 
      :first_name="person.first_name" 
      :last_name="person.last_name" 
      :id="person.id"
      :max_date="person.max_date"
      :topic="person.topic"
      :contact_platform="person.contact_platform"
    />
  </div>
  </div>
</template>
  

  
<style scoped>
/* Your style code here */
</style>