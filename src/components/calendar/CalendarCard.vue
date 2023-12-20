<script>
export default {
    name: 'CalendarCard',
    props: {
        first_name: String,
        last_name: String,
        id: Number,
        max_date: String,
        topic: String,
        contact_platform: String,
    },
}
</script>

<template>
    <div class="calendar-card">
        <div class="calendar-card-name">{{ first_name }} {{ last_name }}</div>
        <div class="calendar-card-date">{{ max_date }}</div>
        <div class="calendar-card-topic">
            <div>Last Topic:</div>
            {{ topic }}
        </div>
    </div>
</template>

<style scoped>
.calendar-card {
    border: 1px solid var(--brunswick-green);
    border-radius: 1rem;

    padding: 0.5rem;
    margin-bottom: 1rem;

    display: grid;
    grid-template-columns: repeat(2, 1fr);
    grid-template-rows: 2rem 1fr;
}
.calendar-card-name {
    grid-area: 1 / 1 / 2 / 2;
}
.calendar-card-date {
    grid-area: 1 / 2 / 2 / 3;
}
.calendar-card-topic {
    grid-area: 2 / 1 / 3 / 3;
    text-align: left;
}
</style>
