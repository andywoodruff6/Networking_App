<script>
export default {
    props: [
        'date',
        'topic',
        'contact_platform',
    ],
}
</script>

<template>
    <div>
        <p>Date: {{ date }}</p>
        <p>Topic: {{ topic }}</p>
        <p>Contact Platform: {{ contact_platform }}</p>
    </div>
</template>

<style scoped>
/* Add your component styles here */
</style>
