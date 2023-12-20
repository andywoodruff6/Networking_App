<script>
export default {
    name: 'EventHistoryCard',
    props: [
        'date',
        'topic',
        'contact_platform',
    ],
}
</script>

<template>
    <div class="event-history-card">
        <div class="event-history-card-date">Date: {{ date }}</div>
        <div class="event-history-card-platform">Platform: {{ contact_platform }}</div>
        <div class="event-history-card-topic">Topic: {{ topic }}</div>
    </div>
</template>

<style scoped>
.event-history-card {
    border: 1px solid var(--brunswick-green);
    border-radius: 1rem;

    padding: 0.5rem;
    margin-bottom: 1rem;

    display: grid;
    grid-template-columns: repeat(2, 1fr);
    grid-template-rows: 2rem 1fr;
}
.event-history-card-date {
    grid-area: 1 / 1 / 2 / 2;
}
.event-history-card-platform {
    grid-area: 1 / 2 / 2 / 3;
}
.event-history-card-topic {
    grid-area: 2 / 1 / 3 / 3;
    text-align: left;
}
</style>
