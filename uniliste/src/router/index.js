import { createRouter, createWebHistory } from "vue-router";
import LoginPage from "../LoginPage.vue";  
import FirstPage from "../FirstPage.vue";

const routes = [
  { path: "/", component: LoginPage },
  { path: "/first", component: FirstPage }
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;