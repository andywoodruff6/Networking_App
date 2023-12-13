<script>
import DetailedPerson from './DetailedPerson.vue'
import DetailedHistory from './DetailedHistory.vue'

export default {
    name: 'FullPersonInfo',
    components: {
        DetailedPerson,
        DetailedHistory,
    },
    props: [
        'id',
        'first_name',
        'last_name',
        'relationship',
        'email',
        'phone_number'
    ],
}
</script>

<template>
    <div class="addPersonPage">
        <div>Full Person Info</div>
        <div class="detailedPage">
            <DetailedPerson :first_name="first_name" :last_name="last_name" :id="id" :relationship="relationship"
                :phone_number="phone_number" :email="email" />
            <DetailedHistory :id="id" />
        </div>

    </div>
</template>

<style scoped>
.detailedPage {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
}
</style>
