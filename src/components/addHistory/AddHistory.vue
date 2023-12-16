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
            // console.log('openMe');
            this.showModal = true;
            document.addEventListener('click', this.handleOutsideClick);
        },
        closeMe() {
            // console.log('closeMe');
            this.showModal = false;
            document.removeEventListener('click', this.handleOutsideClick);
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

<template>
    <div class="addHistory">
        <AddHistoryForm ref="addHistoryForm" v-if="showModal"  @close="showModal = false"/>

        <button class="add-icon-link" @click.stop="openMe()">
            <i class="fa-solid fa-square-plus fa-2xl" />
        </button>
    </div>
</template>
    
<style scoped>
.addHistory {
    background-color: var(--dark-green);
    border-radius: 5px;
}
</style>