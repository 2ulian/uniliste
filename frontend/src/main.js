import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";

const app = createApp(App);
app.use(router);
app.mount("#app");

fetch(`/api/greet?name=${encodeURIComponent("LoginPage")}`)
    .then((resp) => {
        if (!resp.ok) throw new Error(`HTTP ${resp.status} ${resp.statusText}`);
        return resp.text();
    })
    .then((msg) => {
        console.log(msg);
    })
    .catch((e) => {
        console.error("greet failed", e);
    });