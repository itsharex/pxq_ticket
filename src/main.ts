import { createApp } from "vue";
import "./styles.css";
import router from './router.ts'
import App from "./App.vue";
import 'vue3-toastify/dist/index.css';
import Vue3Toastify, { type ToastContainerOptions } from 'vue3-toastify';
const app = createApp(App)
app.use(router)
app.use(Vue3Toastify, {
    autoClose: 3000,
  } as ToastContainerOptions);
app.mount("#app")