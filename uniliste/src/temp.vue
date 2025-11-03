<!-- FICHIER TEMPORAIRE LE TEMPS DE REORGANISER TOUS LES FICHIER ET LES TRIER
     FICHIER CORRESPONDANT A L'AFFICHAGE DES ELEVES -->
<template>
  <nav class="navbar">
      <img src="../assets/unilim.png" alt="Logo" class="logo" />
      <ul>
        <li><router-link to="/prof">retourner au menu</router-link></li>
        <li><router-link to="/list">liste des eleves</router-link></li>
        <li><router-link to="/ajouter-eleve">ajouter un eleve</router-link></li>
        <li><router-link to="/supprimer-eleve">supprimer un eleve</router-link></li>
        <li><router-link to="/modifier-eleve">modifier un eleve</router-link></li>
      </ul>
    </nav>

  <main class="student-list-container">
    <h1>Liste des élèves</h1>

    <div class="filters-container">
      <select v-model="selectedPromo">
        <option value="" disabled>Sélectionner votre promotion</option>
        <option v-for="promo in promos" :key="promo" :value="promo">{{ promo }}</option>
      </select>

      <select v-model="selectedTD">
        <option value="" disabled>Sélectionner votre groupe de TD</option>
        <option v-for="td in tds" :key="td" :value="td">{{ td }}</option>
      </select>
      
      <select v-model="selectedTP">
        <option value="" disabled>Sélectionner votre groupe de TP</option>
        <option v-for="tp in tps" :key="tp" :value="tp">{{ tp }}</option>
      </select>
    </div>

    <div class="student-list">
      <div class="student-list-header">
        <span>Nom / Prénom</span>
        <span>Promo</span>
        <span>Groupe</span>
        <span>Aménagement</span>
        <span>Actions</span>
      </div>

      <div v-for="student in filteredStudents" :key="student.id" class="student-item">
        <span>{{ student.name }}</span>
        <span>{{ student.promo }}</span>
        <span>{{ student.group }}</span>
        <span>{{ student.aménagement }}</span>
        <div class="actions-cell">
          <button @click="modifyStudent(student.id)" class="btn-modify">Modifier</button>
          <button @click="deleteStudent(student.id)" class="btn-delete">Supprimer</button>
        </div>
      </div>

      <p v-if="filteredStudents.length === 0" class="no-results">
        Aucun élève ne correspond aux filtres sélectionnés.
      </p>
    </div>
  </main>
</template>

<script setup>
import { ref, computed } from "vue";
import { useRouter } from "vue-router";

const router = useRouter();

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

const allStudents = ref([
  { id: 1, name: 'MARTIN Lucas', promo: 'A2', group: 'G5-B', aménagement: 'NON' },
  { id: 2, name: 'DURAND Marie', promo: 'A2', group: 'G5-B', aménagement: 'Ordinateur' },
  { id: 3, name: 'PETIT Jean', promo: 'A1', group: 'G1-A', aménagement: 'NON' },
  { id: 4, name: 'DUBOIS Alice', promo: 'A3', group: 'G7-A', aménagement: 'Tiers-temps' },
  { id: 5, name: 'LEROY Tom', promo: 'A1', group: 'G1-B', aménagement: 'NON' },
  { id: 6, name: 'MOREAU Léa', promo: 'A2', group: 'G4-A', aménagement: 'NON' },
  { id: 7, name: 'SHAKUR Tupac', promo: 'A3', group: 'G6-A', aménagement: 'NON' },
]);

const filteredStudents = computed(() => {
  let students = allStudents.value;

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

const modifyStudent = (id) => {
  console.log(`Demande de modification pour l'élève ${id}`);
    //TODO
};

const deleteStudent = (id) => {
  console.log(`Demande de suppression pour l'élève ${id}`);
  //TODO
};

const home = () => {
  router.push("/first");
};
</script>

<style scoped>
/* J'ai fusionné vos deux règles .logo en une seule */
.logo {
  width: 60px;
  height: auto; 
  border-radius: 10px;
  margin: 10px 0; /* Maintient la hauteur de la navbar */
}

.navbar {
  display: flex;
  align-items: center;
  justify-content: space-between; 
  background-color: #791919;
  overflow: hidden;
  font-family: 'Plus Jakarta Sans', sans-serif;
  
  padding: 0 5%; 
  
  box-shadow: 0 2px 8px rgba(86, 0, 0, 0.15);
}

.navbar ul {
  display: flex;
  list-style: none;
  padding: 0;
  margin: 0;
}


.navbar ul li a {
  color: white;
  padding: 24px 18px; 
  text-decoration: none;
  font-weight: 600;
  display: block;
  font-size: 15px;
  text-transform: capitalize; 
  
  position: relative;
  
  
  transition: color 0.3s ease; 
}


.navbar ul li a::after {
  content: ''; 
  position: absolute;
  bottom: 18px; 
  left: 50%; 
  
  width: 0; 
  height: 3px; 
  background: #ffc2c2; 
  
  
  transform: translateX(-50%); 
  
  transition: width 0.3s ease-in-out;
}

.navbar ul li a:hover {
  color: #ffc2c2;
  background-color: transparent;
}

.navbar ul li a:hover::after {
  width: 70%; 
}


.navbar ul li a.router-link-exact-active {
  color: #ffc2c2;
  font-weight: 700;
}

.navbar ul li a.router-link-exact-active::after {
  width: 70%; 
}

.student-list-container {
  width: 90%;
  max-width: 1200px;
  margin: 40px auto;
  color: #791919;
  font-family: 'Plus Jakarta Sans', sans-serif;
}

h1 {
  text-align: center;
  font-weight: 700;
  font-size: 2.5rem;
  margin-bottom: 40px;
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

.student-list {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.student-list-header {
  display: grid;
  grid-template-columns: 2.5fr 1fr 1fr 1.5fr 1.5fr;
  gap: 15px;
  padding: 10px 20px;
  font-weight: 700;
  font-size: 14px;
  color: #555;
  border-bottom: 2px solid #ffdfdf;
}

.student-item {
  display: grid;
  grid-template-columns: 2.5fr 1fr 1fr 1.5fr 1.5fr;
  gap: 15px;
  align-items: center;
  background-color: #fff5f5;
  border-radius: 15px;
  padding: 15px 20px;
  box-shadow: 0 2px 4px rgba(255, 157, 157, 0.3);
  font-weight: 500;
  font-size: 15px;
}

.student-item span:first-child {
  font-weight: 700;
}

.actions-cell {
  display: flex;
  gap: 10px;
}

.btn-modify,
.btn-delete {
  border: none;
  border-radius: 10px;
  padding: 8px 16px;
  color: white;
  cursor: pointer;
  font-weight: bold;
  font-size: 13px;
  transition: opacity 0.2s;
}

.btn-modify {
  background-color: #791919;
}

.btn-delete {
  background-color: #D93030;
}

.btn-modify:hover,
.btn-delete:hover {
  opacity: 0.8;
}

.no-results {
  text-align: center;
  color: #777;
  padding: 20px;
  font-style: italic;
}
</style>