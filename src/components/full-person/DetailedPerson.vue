<script>
// import { editPerson, deletePerson } from '../../services/database.js'

export default {
    name: 'DetailedPerson',
    props: [
        'id',
        'first_name',
        'last_name',
        'relationship',
        'email',
        'phone_number'
    ],
    methods: {

        // editThisPerson() {
        //     editPerson(this.id)
        // },
        // deleteThisPerson() {
        //     deletePerson(this.id)
        // }
    }
}
</script>

<template>
    <div>
        <h2>{{ first_name }} {{ last_name }}</h2>
        <div class="detailed-person-view">
            <div>
                <p>{{ relationship }}</p>
            </div>
            <p>Email: {{ email }}</p>
            <p>Phone: {{ phone_number }}</p>
            <!-- <button @click="editThisPerson">Edit</button> -->
            <!-- <button @click="deleteThisPerson">Delete</button> -->
        </div>
    </div>
</template>

<style scoped>
.detailed-person-view {
    border: 1px solid var(--brunswick-green);
    border-radius: 1rem;
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    padding: 1rem;
}
</style>
