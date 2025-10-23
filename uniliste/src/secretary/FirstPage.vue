<template>
  <head class="navbar">
    <img src="../assets/unilim.png" alt="Logo" class="logo" />
    <ul>
      <router-link to="/annee" id="nav-links">Annee Universitaire</router-link>
      <router-link to="/student" id="nav-links">Importer étudiant</router-link>
      <router-link to="/group" id="nav-links">Importer groupes</router-link>
      <router-link to="/ressources" id="nav-links">Importer ressources</router-link>
      <router-link to="/professor" id="nav-links">Importer professeur</router-link>
      <router-link to="/justification" id="nav-links">Saisir justificatif</router-link>
    </ul>
  </head>

  <div class="content">
    <div class="Etudiants_absents">
      <h2>Etudiants absents</h2>
      <area>
      <ul>
        <li v-for="(ligne, i) in liste" :key="i">{{ ligne }}</li>
      </ul>
      </area>
    </div>
  </div>
</template>

<script setup>
import { ref } from "vue";
const liste = ref(JSON.parse(localStorage.getItem("justificationList") || "[]"));

</script>

<style scoped>
.logo {
  width: 50px;
  border-radius: 10px;
  margin: 10px;
}

.navbar {
  display: flex;
  align-items: center;
  background-color: #791919;
  border: 0.5vmin solid #05060f;
  box-shadow: 0.4rem 0.4rem #05060f;
  overflow: hidden;
  color: black;
}

#nav-links {
  color: white;
  padding: 20px;
  text-decoration: none;
  font-weight: bold;
}

.content {
  display: flex;
  padding: 20px;
  gap: 30px;
}

.Etudiants_absents {
  float: right;
  margin: 20px;
  padding: 10px;
  color: #791919;
  background-color: #fff5f5;
  box-shadow: 0 4px 8px rgba(255, 157, 157, 0.3);
  border-radius: 1rem;
  width: 550px;
  height: 500px;
}
</style>
