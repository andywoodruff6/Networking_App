import { createRouter, createWebHistory } from 'vue-router'
import homePage from './pages/homePage.vue'
import calendarPage from './pages/calendarPage.vue'
import networkPage from './pages/networkPage.vue'
import friendPage from './pages/friendPage.vue'
import workPage from './pages/workPage.vue'
import HobbyPage from './pages/HobbyPage.vue'
import addPersonPage from './pages/addPersonPage.vue'
import FullPersonInfo from './components/full-person/FullPersonInfo.vue'

const routes = [
    { path: '/', component: homePage },
    { path: '/calendar', component: calendarPage },
    { path: '/network', component: networkPage },
    { path: '/friend', component: friendPage },
    { path: '/work', component: workPage },
    { path: '/hobby', component: HobbyPage },
    { path: '/addPerson', component: addPersonPage },
    { path: '/fullPersonInfo', component: FullPersonInfo }
]

const router = createRouter({
    history: createWebHistory(),
    routes
})

export default router