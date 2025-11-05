import { createRouter, createWebHistory } from "vue-router";
import LoginPage from "../pages/LoginPage.vue";

import SecretaryFirstPage from "../pages/secretary/FirstPage.vue";
import YearPage from "../pages/secretary/YearPage.vue";
import ImportStudentPage from "../pages/secretary/ImportStudentPage.vue";
import ImportGroupPage from "../pages/secretary/ImportGroupPage.vue";
import ImportProfessorPage from "../pages/secretary/ImportProfessorPage.vue";
import ImportRessourcesPage from "../pages/secretary/ImportRessourcesPage.vue";
import JustificationPage from "../pages/secretary/JustificationPage.vue";
import ListPage from "../pages/temp.vue";

import ProfessorFirstPage from "../pages/professor/FirstPage.vue";
import GroupPage from "../pages/GroupPage.vue";
import CoursPage from "../pages/CoursPage.vue";
import CMPage from "../pages/CMPage.vue";
import RessourcePage from "../pages/RessourcePage.vue";
import AboutPage from "../pages/AboutPage.vue";

const routes = [
  { path: "/", component: LoginPage },

  { path: "/first", component: SecretaryFirstPage },
  {path: "/annee", component: YearPage },
  {path: "/student", component: ImportStudentPage },
  {path: "/group", component: ImportGroupPage },
  {path: "/professor", component: ImportProfessorPage },
  {path: "/ressources", component: ImportRessourcesPage },
  {path: "/justification", component: JustificationPage },

  {path: "/prof", component: ProfessorFirstPage },
  { path: "/groups", component: GroupPage },
  { path: "/cours", component: CoursPage },
  { path: "/ressources", component: RessourcePage },
  { path: "/about", component: AboutPage },
  { path: "/CM", component: CMPage },
  { path: "/list", component: ListPage }

];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;