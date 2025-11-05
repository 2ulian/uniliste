import { createRouter, createWebHistory } from "vue-router";
import LoginPage from "../LoginPage.vue";

import SecretaryFirstPage from "../secretary/FirstPage.vue";
import YearPage from "../secretary/YearPage.vue";
import ImportStudentPage from "../secretary/ImportStudentPage.vue";
import ImportGroupPage from "../secretary/ImportGroupPage.vue";
import ImportProfessorPage from "../secretary/ImportProfessorPage.vue";
import ImportRessourcesPage from "../secretary/ImportRessourcesPage.vue";
import JustificationPage from "../secretary/JustificationPage.vue";
import ListPage from "../temp.vue";

import ProfessorFirstPage from "../professor/FirstPage.vue";
import GroupPage from "../GroupPage.vue";
import CoursPage from "../CoursPage.vue";
import CMPage from "../CMPage.vue";
import RessourcePage from "../RessourcePage.vue";
import AboutPage from "../AboutPage.vue";

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