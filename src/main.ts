import { createApp } from "vue";
import App from "./App.vue";
import './assets/css/main.css';
import { i18n } from './i18n';

createApp(App).use(i18n).mount("#app");
