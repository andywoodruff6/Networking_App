<template>
    <div class="addHistory">
        <AddHistoryForm v-if="showModal"  @close="showModal = false"/>

        <button class="add-icon-link" @click="openMe()">
            <i class="fa-solid fa-square-plus fa-2xl" />
        </button>
    </div>
</template>
    
<script>
import AddHistoryForm from './AddHistoryForm.vue';

export default {
    name: 'AddHistory',
    components: {
        AddHistoryForm
    },
    data() {
        return {
            showModal: false
        };
    },
    methods: {
        openMe() {
            console.log('openMe');
            this.showModal = true;
        },
        handleOutsideClick(e) {
            const formElement = this.$refs.addHistoryForm.$el;
            if (!formElement.contains(e.target)) {
                this.showModal = false;
            }
        }
    },
}
</script>

<style scoped>
/* Add your styles here */
</style>