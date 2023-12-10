<template>
    <div class="addPersonPage">
        <div>Full Person Info</div>
        <div>
            <DetailedPerson />
            <DetailedHistory />
        </div>

    </div>
</template>

<script>
import DetailedPerson from './DetailedPerson.vue'
import DetailedHistory from './DetailedHistory.vue'

export default {
    components: {
        DetailedPerson,
        DetailedHistory,
    }
}

</script>

<style scoped></style>
