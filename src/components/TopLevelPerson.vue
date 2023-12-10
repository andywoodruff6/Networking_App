<template>
    <router-link to="/FullPersonInfo">
    <div>
        {{ first_name }} {{ last_name }} {{ id }}
    </div>
</router-link>
</template>

<script>
export default {
    props: {
        first_name: {
            type: String,
        },
        last_name: {
            type: String,
        },
        id: {
            type: Number,
        },
    }
}
</script>

<style scoped>

</style>
