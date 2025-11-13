<template>
  <nav class="navbar">
    <img src="../../assets/unilim.png" alt="Logo" class="logo" />
    <ul>
      <li><router-link to="/annee" id="nav-links">Année Universitaire</router-link></li>
      <li><router-link to="/student" id="nav-links">Importer étudiant</router-link></li>
      <li><router-link to="/group" id="nav-links">Importer groupes</router-link></li>
      <li><router-link to="/ressources" id="nav-links">Importer ressources</router-link></li>
      <li><router-link to="/professor" id="nav-links">Importer professeur</router-link></li>
      <li><router-link to="/justification" id="nav-links">Saisir justificatif</router-link></li>
      <li><router-link to="/" id="nav-links">Quitter</router-link></li>
    </ul>
  </nav>

  <div class="content">
    <div class="Etudiants_absents">
      <h2>Absents </h2>
      <ul>
        <li v-for="(etudiant, index) in liste" :key="index">
          {{ etudiant.nom }} {{ etudiant.prenom }} {{ etudiant.promo }} {{ etudiant.groupe }}
        </li>
      </ul>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';

const liste = ref([]);

onMounted(() => {
  const data = localStorage.getItem('justificationList');
  if (data) {
    try {
      const parsed = JSON.parse(data);
      if (Array.isArray(parsed)) {
        liste.value = parsed;
      } else {
        console.error('Les données dans localStorage ne sont pas un tableau');
      }
    } catch (error) {
      console.error('Erreur lors du parsing JSON:', error);
    }
  } else {
    console.log('Aucune donnée justificationList dans le localStorage');
  }
});
</script>


<style scoped>
.content {
  display: flex;
  justify-content: flex-start;
  padding: 20px;
}

.Etudiants_absents {
  background-color: #fff5f5;
  border-radius: 10px;
  padding: 20px;
  width: 400px;
  color: #791919;
}

.Etudiants_absents h2 {
  text-align: center;
  margin-bottom: 15px;
}

.Etudiants_absents ul {
  list-style: none;
  padding: 0;
}

.Etudiants_absents li {
  margin-bottom: 10px;
  padding: 8px;
  background-color: #ffdede;
  border-radius: 5px;
}
</style>
