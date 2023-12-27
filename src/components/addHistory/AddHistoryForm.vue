<script>
import { addHistory } from '../../services/database.js';
import { getPeople } from '../../services/database.js';
import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/api/notification';

export default {
    name: 'AddHistoryForm',
    data() {
        return {
            person_id: '',
            date: '',
            topic: '',
            contact_platform: '',
            people: [],
        }
    },
    mounted() {
        this.fetchPeople();
    },
    methods: {
        closeMe() {
            this.$emit('close');
        },
        async fetchPeople() {
            try {
                console.log('fetchPeople triggered');
                this.people = await getPeople();
            } catch (error) {
                console.error(error);
            }
        },
        async SubmitEvent() {
            try {
                addHistory(
                    this.person_id,
                    Date.now(),
                    this.topic,
                    this.contact_platform,
                );
                // send notification
                let permissionGranted = await isPermissionGranted();
                console.log("permission is:", permissionGranted)
                if (!permissionGranted) {
                    const permission = await requestPermission();
                    permissionGranted = permission === 'granted';
                }
                if (permissionGranted) {
                    sendNotification({ title: 'Add Event Successful', body: 'Event logged for later viewing.' });
                }
            } catch (error) {
                console.error(error);
            }
            this.closeMe();
        },
    }
}
</script>

<template>
    <div>
        <!-- Begin Form -->
        <div class="input-block">
            <div class="input-card">
                <label for="person_id">Person</label>
                <select id="person_id" v-model="person_id">
                    <option value="">Select...</option>
                    <option v-for="person in people" :key="person.id" :value="person.id">
                        {{ person.first_name }} {{ person.last_name }}
                    </option>
                </select>
            </div>
            <div class="input-card">
                <label for="contact_platform">Contact Platform</label>
                <input type="text" id="contact_platform" v-model="contact_platform" placeholder="How?" />
            </div>
            <div class="input-card">
                <label for="topic">Topic</label>
                <input type="text" id="topic" v-model="topic" placeholder="Conversation Notes" />
            </div>

            <div class="add-history-form-button">
                <button class="button" @click="SubmitEvent">Add Event</button>
            </div>
        </div>
        <!-- End Form -->
    </div>
</template>

<style scoped>
.add-history-form-button {
    display: flex;
    justify-content: right;
}
</style>
