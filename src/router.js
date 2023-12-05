import { createRouter, createWebHistory } from 'vue-router'
import homePage from './pages/homePage.vue'
import anotherPage from './pages/anotherPage.vue'

const routes = [
    { path: '/', component: homePage },
    { path: '/another', component: anotherPage }
]

const router = createRouter({
    history: createWebHistory(),
    routes
})

export default router