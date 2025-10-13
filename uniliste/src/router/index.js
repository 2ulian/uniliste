import { createRouter, createWebHistory } from "vue-router";
import LoginPage from "../LoginPage.vue";

import FirstPage from "../secretary/FirstPage.vue";
import YearPage from "../secretary/YearPage.vue";
import ImportStudent from "../secretary/ImportStudent.vue";

import GroupPage from "../GroupPage.vue";
import CoursPage from "../CoursPage.vue";
import CMPage from "../CMPage.vue";
import RessourcePage from "../RessourcePage.vue";
import AboutPage from "../AboutPage.vue";
import { path } from "@tauri-apps/api";

const routes = [
  { path: "/", component: LoginPage },

  { path: "/first", component: FirstPage },
  {path: "/annee", component: YearPage },
  {path: "/student", component: ImportStudent },

  { path: "/groups", component: GroupPage },
  { path: "/cours", component: CoursPage },
  { path: "/ressources", component: RessourcePage },
  { path: "/about", component: AboutPage },
  { path: "/CM", component: CMPage }
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;