<template>
    <div>
        <h3>{{ first_name }} {{ last_name }}</h3>
        <div>
            <div>
                <p>Relationship:</p>
                <p>{{ relationship }}</p>
            </div>
            <p>Email: {{ email }}</p>
            <p>Phone Number: {{ phone_number }}</p>
            <!-- <button @click="editThisPerson">Edit</button> -->
            <!-- <button @click="deleteThisPerson">Delete</button> -->
        </div>
    </div>
</template>

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

<style scoped></style>

