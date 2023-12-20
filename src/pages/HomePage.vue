<script >
import CalendarCard from '../components/calendar/CalendarCard.vue';
import { testDBCalendar, calendarByPosition } from '../services/database';

export default {
  name: 'homePage',
  components: {
    CalendarCard,
  },
  data() {
    return {
      calendarArray: [],
      lowestDateArray: [],
      secondLowestDateArray: [],
    };
  },
  mounted() {
    this.fetchCalendar();
    this.fetchCalendarByPosition();
  },
  methods: {
    async fetchCalendar() {
      try {
        this.calendarArray = await testDBCalendar();

      } catch (error) {
        console.error(error);
      }
    },
    // need to grab just the lowest date
    async fetchCalendarByPosition() {
      try {
        this.lowestDateArray = await calendarByPosition(0);
        this.secondLowestDateArray = await calendarByPosition(1);

      } catch (error) {
        console.error(error);
      }
    },
  }
}
</script>

<template>
  <div class="homePage">
    <h1 class="div1">Welcome to Connecti</h1>
    <div class="div2 calendar-card-home">
      <CalendarCard 
        :first_name="lowestDateArray.first_name" 
        :last_name="lowestDateArray.last_name"
        :id="lowestDateArray.id" 
        :max_date="lowestDateArray.max_date" 
        :topic="lowestDateArray.topic"
        :contact_platform="lowestDateArray.contact_platform"
      />
        
    </div>
    <div class="div3 calendar-card-home">
      <CalendarCard 
        :first_name="secondLowestDateArray.first_name" 
        :last_name="secondLowestDateArray.last_name"  
        :id="secondLowestDateArray.id" 
        :max_date="secondLowestDateArray.max_date" 
        :topic="secondLowestDateArray.topic"
        :contact_platform="secondLowestDateArray.contact_platform" 
      />
    </div>
    <div class="div4 toolTip">
      <div>Tool Tip:</div>
      Add new connection with the person plus icon.
    </div>
    <div class="div5 toolTip">
      <div>Tool Tip:</div>
      Friends, Work or Hobby icons show just that group.
    </div>
    <div class="div6 toolTip">
      <div>Tool Tip:</div>
      Click the plus in the bottom right to add a contact event.
    </div>
  </div>
</template>

<style scoped>
.homePage {
  display: grid;
  grid-template-columns: repeat(6, 1fr);
  grid-template-rows: 25% 1fr 13%;
  grid-column-gap: 0px;
  grid-row-gap: 0px;
  height: 90vh;
}

.toolTip {
  padding: 7%;
}
.calendar-card-home {
  margin: 0.5rem;
}

.div1 {
  grid-area: 1 / 1 / 2 / 7;
}

.div2 {
  grid-area: 2 / 1 / 3 / 4;
}

.div3 {
  grid-area: 2 / 4 / 3 / 7;
}

.div4 {
  grid-area: 3 / 1 / 4 / 3;
}

.div5 {
  grid-area: 3 / 3 / 4 / 5;
}

.div6 {
  grid-area: 3 / 5 / 4 / 7;
}
</style>
