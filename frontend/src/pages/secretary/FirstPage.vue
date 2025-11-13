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
    <div class="Students_not_present">
      <h2>Eleves Absents </h2>
      <ul>
        <li v-for="(student, index) in liste" :key="index">
          {{ student.name }}    {{ student.promo }} {{ student.group }}
        </li>
      </ul>
    </div>
    <div class="student_call">
      <div class="student-list">
        <h2>Liste des élèves</h2>
        <div class="filters-container">
          <select v-model="selectedPromo">
            <option value="" >Toutes les promos</option>
            <option v-for="promo in promos" :key="promo" :value="promo">{{ promo }}</option>
          </select>

          <select v-model="selectedTD">
            <option value="" >Tous les TD</option>
            <option v-for="td in tds" :key="td" :value="td">{{ td }}</option>
          </select>
          
          <select v-model="selectedTP">
            <option value="" >Tous les TP</option>
            <option v-for="tp in tps" :key="tp" :value="tp">{{ tp }}</option>
          </select>
        </div>
        <div class="student-list-header">
          <span>Nom / Prénom</span>
          <span>Promo</span>
          <span>Groupe</span>
          <span>Aménagement</span>
          <span>Absent</span>
        </div>

      <div v-for="(student, index) in filteredStudents" :key="index" class="student-item">
        <span>{{ student.name }}</span>
        <span>{{ student.promo }}</span>
        <span>{{ student.group }}</span>
        <span>{{ student.amenagement }}</span>
        <div class="actions-cell">
          <input type="checkbox">
        </div>
      </div>
      <div class="register_save">
        <button>enregistrer</button>
      </div>
      <!--<p v-if="allStudents.length === 0" class="no-results">
        Aucun élève ne correspond aux filtres sélectionnés.
      </p>-->
    </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, computed } from 'vue';

const liste = ref([]);

const selectedPromo = ref("");
const selectedTD = ref("");
const selectedTP = ref("");

const promos = ['A1', 'A2', 'A3'];
const tds = ['G1', 'G2', 'G3', 'G4', 'G5', 'G6', 'G7'];
const tps = [];
for (let i = 1; i <= 7; i++) {
  tps.push(`G${i}-A`);
  tps.push(`G${i}-B`);
}

onMounted(() => {
const data = localStorage.getItem('studentsList');
  if (data) {
    try {
      const parsed = JSON.parse(data);
      if (Array.isArray(parsed)) {
        liste.value = parsed;
      } 
    } catch (error) {
      console.error('Erreur lors du parsing JSON:', error);
    }
  } else {
    console.log('Aucun eleve absent');
  }
});

const filteredStudents = computed(() => {
  let students = liste.value;
  if (selectedPromo.value) {
    students = students.filter(s => s.promo === selectedPromo.value);
  }

  if (selectedTD.value) {
    students = students.filter(s => s.group.startsWith(selectedTD.value));
  }

  if (selectedTP.value) {
    students = students.filter(s => s.group === selectedTP.value);
  }
  return students;
});
</script>

<style scoped>
.content {
  display: inline-flex;
  padding: 20px;
}

.Students_not_present {
  background-color: #fff5f5;
  border-radius: 10px;
  padding: 20px;
  width: 400px;
  color: #791919;
  margin-right: 100px;
}

.Students_not_present h2 {
  text-align: center;
  margin-bottom: 15px;
}

.Students_not_present ul {
  list-style: none;
  padding: 0;
}

.Students_not_present li {
  margin-bottom: 10px;
  padding: 20px;
  text-align: justify;
  background-color: #ffdede;
  border-radius: 14px;
}

.student-list-container {
  width: 90%;
  max-width: 1200px;
  margin: 40px auto;
  color: #791919;
  font-family: 'Plus Jakarta Sans', sans-serif;
}

.header-bar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 16px;
  flex-wrap: wrap;
}

.import-btn {
  background: #791919;
  color: white;
}

h1 {
  text-align: center;
  font-weight: 700;
  font-size: 2.5rem;
  margin-bottom: 40px;
}

.student-list {
  display: flex;
  flex-direction: column;
  gap: 10px;
}
.student-list h2 {
  text-align: center;
  margin-bottom: 15px;
  color: #791919;
}

.filters-container {
  display: flex;
  justify-content: center;
  flex-wrap: wrap;
  gap: 20px;
  margin-bottom: 40px;
}
select {
  background-color: #fff5f5;
  border-radius: 20px; 
  padding: 12px 18px;
  font-family: inherit;
  color: #791919;
  font-size: 14px;
  min-width: 280px; 
  cursor: pointer;
  border: none;
}

select option[value=""] {
  color: #a9a9a9;
}

select:focus {
  background-color: #ffffff;
  border-color: #ffd6d6;
  outline: none;
  box-shadow: 0 0 0 3px rgba(255, 200, 200, 0.2);
}

.student-list-header {
  display: grid;
  grid-template-columns: 2.5fr 1fr 1fr 1.5fr 1.5fr;
  gap: 20px;
  padding: 14px 24px;
  font-weight: 700;
  font-size: 15px;
  color: #555;
  border-bottom: 3px solid #ffdfdf;
  background-color: #ffe5e5;
  border-radius: 12px;
}

.student-item {
  display: grid;
  grid-template-columns: 2.5fr 1fr 1fr 1.5fr 1.5fr;
  gap: 20px;
  align-items: center;
  background-color: #fff5f5;
  border-radius: 18px;
  padding: 18px 24px;
  box-shadow: 0 3px 8px rgba(255, 130, 130, 0.35);
  font-weight: 500;
  font-size: 16px;
  transition: background-color 0.3s ease;
}
.student-item:hover {
  background-color: #ffdada;
  box-shadow: 0 6px 15px rgba(255, 110, 110, 0.5);
}

.student-item span:first-child {
  font-weight: 700;
}

.actions-cell {
  display:flex;
  float: right;
  gap: 12px;
}


.no-results {
  text-align: center;
  color: #777;
  padding: 24px;
  font-style: italic;
  font-size: 16px;
}
</style>
