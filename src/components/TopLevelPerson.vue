<template>
    <router-link :to="{ name: 'FullPersonInfo', params: { 
        id: id,
        first_name: first_name,
        last_name: last_name,
        relationship: relationship,
        email: email,
        phone_number: phone_number
        }, }">
        <div>
            {{ first_name }} {{ last_name }} {{ id }}
        </div>
    </router-link>
</template>

<script>
export default {
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
