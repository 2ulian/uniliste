<template>
  <nav class="navbar">
      <img src="../../assets/unilim.png" alt="Logo" class="logo" />
      <ul>
        <li><router-link to="/prof">Acceuil</router-link></li>
        <li><router-link to="/sudentlist">liste des élèves</router-link></li>
        <li><router-link to="/callsheet">Listes d'appel</router-link></li>
        <li><router-link to="/stafflist">Liste du personnel</router-link></li>
        <li><router-link to="/">Se déconnecter</router-link></li>
      </ul>
    </nav>

  <main class="student-list-container">
    <h1>Liste des élèves</h1>

    <div class="filters-container">
      <select v-model="selectedPromo">
        <option v-for="promo in promos" :key="promo" :value="promo">{{ promo }}</option>
      </select>

      <select v-model="selectedTD">
        <option v-for="td in tds" :key="td" :value="td">{{ td }}</option>
      </select>
      
      <select v-model="selectedTP">
        <option v-for="tp in tps" :key="tp" :value="tp">{{ tp }}</option>
      </select>
    </div>

    <div class="student-list">
      <div class="student-list-header">
        <span>Nom / Prénom</span>
        <span>Promo</span>
        <span>Groupe</span>
        <span>Aménagement</span>
        <span>Statut</span>
      </div>

      <div v-for="student in filteredStudents" :key="student.id" class="student-item">
        <span>{{ student.name }}</span>
        <span>{{ student.promo }}</span>
        <span>{{ student.group }}</span>
        <span>{{ student.aménagement }}</span>
        <span class="status-toggle" :class="student.status === 'Présent' ? 'status-green' : 'status-red'"> {{ student.status }}</span>
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

const selectedPromo = ref("Toutes les promotions");
const selectedTD = ref("Tous les groupes de TD");
const selectedTP = ref("Tous les groupes de TP");

const promos = ['Toutes les promotions', 'A1', 'A2', 'A3'];
const tds = ['Tous les groupes de TD', 'G1', 'G2', 'G3', 'G4', 'G5', 'G6', 'G7'];
const tps = ['Tous les groupes de TP'];
for (let i = 1; i <= 7; i++) {
  tps.push(`G${i}-A`);
  tps.push(`G${i}-B`);
}

const allStudents = ref([
  { id: 1, name: 'MARTIN Lucas', promo: 'A2', group: 'G5-B', aménagement: 'NON', status: 'Absent' },
  { id: 2, name: 'DURAND Marie', promo: 'A2', group: 'G5-B', aménagement: 'Ordinateur', status: 'Présent' },
  { id: 3, name: 'PETIT Jean', promo: 'A1', group: 'G1-A', aménagement: 'NON', status: 'Présent' },
{ id: 4, name: 'DUBOIS Alice', promo: 'A3', group: 'G7-A', aménagement: 'Tiers-temps', status: 'Présent' },
  { id: 5, name: 'LEROY Tom', promo: 'A1', group: 'G1-B', aménagement: 'NON', status: 'Présent' },
  { id: 6, name: 'MOREAU Léa', promo: 'A2', group: 'G4-A', aménagement: 'NON', status: 'Absent' },
  { id: 7, name: 'SHAKUR Tupac', promo: 'A3', group: 'G6-A', aménagement: 'NON', status: 'Présent' },
]);

const filteredStudents = computed(() => {
  let students = allStudents.value;

  if (selectedPromo.value && selectedPromo.value !== 'Toutes les promotions') {
    students = students.filter(s => s.promo === selectedPromo.value);
  }

  if (selectedTD.value && selectedTD.value !== 'Tous les groupes de TD') {
    students = students.filter(s => s.group.startsWith(selectedTD.value));
  }

  if (selectedTP.value && selectedTP.value !== 'Tous les groupes de TP') {
    students = students.filter(s => s.group === selectedTP.value);
  }

  return students;
});

const home = () => {
  router.push("/first");
};
</script>

<style scoped>
    .logo {
    width: 60px;
    height: auto; 
    border-radius: 10px;
    margin: 10px ;
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
    font-size: 40px;
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
    grid-template-columns: 2.5fr 1fr 1fr 1.5fr 1fr;
    gap: 15px;
    padding: 10px 20px;
    font-weight: 700;
    font-size: 14px;
    color: #791919;
    border-bottom: 2px solid #ffdfdf;
    }

    .student-item {
    display: grid;
    grid-template-columns: 2.5fr 1fr 1fr 1.5fr 1fr;
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

    .no-results {
    text-align: center;
    color: #791919;
    padding: 20px;
    font-style: italic;
    }

    .status-toggle {
    font-weight: 700;
    justify-self: end;
    padding: 4px 8px;
    border-radius: 5px;
    }

    .status-red {
    color: #D93030;
    }

    .status-green {
    color: #28a745;
    }
</style>