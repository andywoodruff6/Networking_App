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
        </div>
    </div>
</template>

<script>
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
}

</script>

<style scoped></style>

