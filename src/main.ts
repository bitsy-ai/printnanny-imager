import { createApp } from "vue";
import { createPinia } from "pinia";
import piniaPluginPersistedstate from "pinia-plugin-persistedstate";

const pinia = createPinia();
pinia.use(piniaPluginPersistedstate);
import router from "./router";
import "./style.css";
import App from "./App.vue";

const app = createApp(App);
app.use(pinia);
app.use(router);
app.mount("#app");
