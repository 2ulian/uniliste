import { createRouter, createWebHistory } from "vue-router";
import LoginPage from "../LoginPage.vue";
import FirstPage from "../FirstPage.vue";
import GroupPage from "../GroupPage.vue";
import CoursPage from "../CoursPage.vue";
import RessourcePage from "../RessourcePage.vue";

const routes = [
  { path: "/", component: LoginPage },
  { path: "/first", component: FirstPage },
  { path: "/groups", component: GroupPage },
  { path: "/cours", component: CoursPage },
  { path: "/ressources", component: RessourcePage }
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;