import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
import naive from 'naive-ui'
import { createRouter, createWebHashHistory } from 'vue-router'
import Search from './pages/Search.vue'
import PlayList from './pages/PlayList.vue'
import Setting from './pages/Setting.vue'
import { createPinia } from 'pinia'
import eventBus from 'vue3-eventbus'

import piniaPluginPersistedstate from 'pinia-plugin-persistedstate'
const routes = [
    {
        path: '/',
        name:'search',
        component: Search,
        meta: {
            keepAlive: true
        },
    },
    // {
    //     path: '/player',
    //     name:'player',
    //     component: Player,
    //     meta: {
    //         keepAlive: true
    //     },
    // },
    {
        path: '/playlist', 
        name:'playlist',
        component: PlayList,
        meta: {
            keepAlive: true
        }
    },
    {
        path: '/setting', 
        name:'setting',
        component: Setting,
        meta: {
            keepAlive: true
        }
    }
]

const router = createRouter({
    history: createWebHashHistory(),
    routes
})
const app = createApp(App);
const pinia = createPinia();
pinia.use(piniaPluginPersistedstate);
app.use(eventBus);
app.use(pinia);
app.use(naive);
app.use(router);
app.mount('#app');