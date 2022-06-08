import { createApp } from "vue";
import App from "./App.vue";
import router from "./src/router";

const app = createApp(App);

app.use(router);

app.mount("#app");
