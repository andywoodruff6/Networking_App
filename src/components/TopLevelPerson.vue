<script>
export default {
    name: 'TopLevelPerson',
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
    <router-link :to="{
        name: 'FullPersonInfo', params: {
            id: id,
            first_name: first_name,
            last_name: last_name,
            relationship: relationship,
            email: email,
            phone_number: phone_number
        },
    }">
        <h3>
            {{ first_name }} {{ last_name }}
        </h3>
    </router-link>
</template>

<style scoped>
/* .display-name {
    color: white;
    text-decoration: none;
} */
</style>
