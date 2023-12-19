<script>
import { getHistoryById } from '../../services/database.js'
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
                this.history = await getHistoryById(this.id);
            } catch (error) {
                console.error(error);
            }
        },
    },
}
</script>

<template>
    <div class="detailed-history-view">
        <div v-for="event in history" :key="event.id">
            <EventHistoryCard 
                :date="event.date" 
                :topic="event.topic" 
                :contact_platform="event.contact_platform" 
            />
        </div>
    </div>
</template>



<style scoped>
.detailed-history-view {
    margin-left: 1.5rem;
}
</style>