<script>
import {getHistory} from '../../services/database.js'
import EventHistoryCard from './EventHistoryCard.vue'

export default {
    name: 'DetailedHistory',
    components: {
        EventHistoryCard,
    },
    props: [
        'id',
    ],
    data() {
        return {
            history: [],
        };
    },
    mounted() {
        this.fetchHistory();
    },
    methods: {
        async fetchHistory() {
            try {
                this.history = await getHistory(this.id);
            } catch (error) {
                console.error(error);
            }
        },
    },
}
</script>

<template>
    <div v-for="event in history" :key="event.id">
        <EventHistoryCard
            :date="event.date"
            :topic="event.topic"
            :contact_platform="event.contact_platform"
        />
    </div>
</template>



<style scoped></style>