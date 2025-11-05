<template>
  <nav class="navbar">
    <img src="../assets/unilim.png" alt="Logo" class="logo" />
    <ul>
      <li><router-link to="/first" id="nav-links">Menu</router-link></li>
      <li><router-link to="/annee" id="nav-links">Année Universitaire</router-link></li>
      <li><router-link to="/student" id="nav-links">Importer étudiant</router-link></li>
      <li><router-link to="/group" id="nav-links">Importer groupes</router-link></li>
      <li><router-link to="/ressources" id="nav-links">Importer ressources</router-link></li>
      <li><router-link to="/professor" id="nav-links">Importer professeur</router-link></li>
      <li><router-link to="/" id="nav-links">Quitter</router-link></li>
    </ul>.logo {
  width: 60px;
  height: auto;
  border-radius: 10px;
  margin: 10px;
}

.navbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  background-color: #791919;
  overflow: hidden;
  font-family: "Plus Jakarta Sans", sans-serif;

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
  content: "";
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
  font-family: "Plus Jakarta Sans", sans-serif;
}

h1 {
  text-align: center;
  font-weight: 700;
  font-size: 2.5rem;
  margin-bottom: 20px;
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
  grid-template-columns: 2.5fr 1fr 1fr 1.5fr;
  gap: 15px;
  padding: 10px 20px;
  font-weight: 700;
  font-size: 14px;
  color: #555;
  border-bottom: 2px solid #ffdfdf;
}

.student-item {
  display: grid;
  grid-template-columns: 2.5fr 1fr 1fr 1.5fr;
  gap: 15px;
  align-items: center;
  background-color: #fff5f5;
  border-radius: 15px;
  padding: 15px 20px;
  box-shadow: 0 2px 4px rgba(255, 157, 157, 0.3);
  font-weight: 500;
  font-size: 15px;
}

.student
  </nav>

  <main class="student-list-container">
    <div style="display:flex; justify-content:space-between; align-items:center; gap:16px; flex-wrap:wrap;">
      <h1>Élèves absents</h1>
      <div>
        <button class="custom-file-upload btn-modify" @click="openExplorer" style="background:#791919; color:white;">
          📂 Importer un fichier CSV 
        </button>
        <input ref="fileInput" type="file" @change="onFileChange" accept=".csv" hidden />
      </div>
    </div>

    <!-- FILTRES -->
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

    <!-- LISTE ÉLÈVES -->
    <div class="student-list">
      <div class="student-list-header">
        <span>Nom / Prénom</span>
        <span>Promo</span>
        <span>Groupe</span>
        <span>Justificatif</span>
      </div>

      <div v-for="student in filteredStudents" :key="student.id" class="student-item">
        <span>{{ student.name }}</span>
        <span>{{ student.promo }}</span>
        <span>{{ student.group }}</span>

        <div class="actions-cell">
          <input type="file" accept="image/png" @change="onUploadJustif($event, student.id)" />
          <button v-if="student.justificatif" class="btn-modify" @click="viewJustif(student.id)">
            Voir justificatif
          </button>
        </div>
      </div>

      <p v-if="filteredStudents.length === 0" class="no-results">
        Aucun élève absent.
      </p>
    </div>
  </main>
</template>

<script setup>
import { ref, computed, onMounted } from "vue";

const fileInput = ref(null);
const STORAGE_KEY = "studentsList_v1";
const allStudents = ref([]);

// Filtres sélectionnés
const selectedPromo = ref("");
const selectedTD = ref("");
const selectedTP = ref("");

// Listes fixes (à ajuster selon besoin)
const promos = ['A1', 'A2', 'A3'];
const tds = ['G1', 'G2', 'G3', 'G4', 'G5', 'G6', 'G7'];
const tps = [];
for (let i = 1; i <= 7; i++) {
  tps.push(`G${i}-A`);
  tps.push(`G${i}-B`);
}

onMounted(() => {
  const raw = localStorage.getItem(STORAGE_KEY);
  if (raw) {
    try {
      const parsed = JSON.parse(raw);
      if (Array.isArray(parsed)) allStudents.value = parsed;
    } catch (e) {
      console.warn("Erreur parsing localStorage:", e);
    }
  }
});

// Filtrage selon promo, TD, TP et absent = true
const filteredStudents = computed(() => {
  let students = allStudents.value.filter(s => s.absent);

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

const openExplorer = () => fileInput.value && fileInput.value.click();

function saveToStorage() {
  localStorage.setItem(STORAGE_KEY, JSON.stringify(allStudents.value));
}

function normalizeCell(cell) {
  if (typeof cell !== "string") return "";
  let c = cell.trim();
  if ((c.startsWith('"') && c.endsWith('"')) || (c.startsWith("'") && c.endsWith("'"))) {
    c = c.slice(1, -1);
  }
  return c.trim();
}

const splitCommaAware = (line) => {
  return line.split(/,(?=(?:[^"]*"[^"]*")*[^"]*$)/);
};

function mapHeaderIndex(headerArr) {
  const norm = (s) => s.toLowerCase().normalize('NFD').replace(/[\u0300-\u036f]/g, "").replace(/\s+/g, "");
  const h = headerArr.map(hd => norm(normalizeCell(hd)));

  return {
    nom: h.indexOf('nom'),
    prenom: h.indexOf('prenom'),
    nomcomplet: h.indexOf('nomcomplet'),
    name: h.indexOf('name'),
    promo: h.indexOf('promo'),
    groupe: h.indexOf('groupe'),
    group: h.indexOf('group')
  };
}

function parseCSVText(text) {
  const lines = text.split(/\r?\n/).map(l => l.trim()).filter(l => l !== "");
  if (lines.length === 0) return [];

  // Remplacer splitCommaAware par splitSemicolonAware si ton CSV est séparé par des ;
  const headerCols = splitSemicolonAware(lines[0]).map(normalizeCell);
  const mapIdx = mapHeaderIndex(headerCols);

  const rows = [];
  for (let i = 1; i < lines.length; i++) {
    const cols = splitSemicolonAware(lines[i]).map(normalizeCell);

    const prenom = mapIdx.prenom !== -1 ? cols[mapIdx.prenom] : "";
    const nom = mapIdx.nom !== -1 ? cols[mapIdx.nom] : "";

    const promo = mapIdx.promo !== -1 ? cols[mapIdx.promo] : "";
    const group = (mapIdx.groupe !== -1 ? cols[mapIdx.groupe] : (mapIdx.group !== -1 ? cols[mapIdx.group] : ""));

    if (nom || prenom) {
      rows.push({
        id: rows.length + 1,
        nom,
        prenom,
        name: `${nom} ${prenom}`,
        promo,
        group,
        absent: true,
        justificatif: null,
      });
    }
  }
  return rows;
}


function readFileAsTextWithEncoding(file, encoding = "utf-8") {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = (e) => resolve(e.target.result);
    reader.onerror = reject;
    reader.readAsText(file, encoding);
  });
}

async function onFileChange(event) {
  const file = event.target.files ? event.target.files[0] : event;
  console.log("Fichier sélectionné :", file);
  if (!file) {
    alert("Aucun fichier sélectionné.");
    return;
  }

  try {
    let text = await readFileAsTextWithEncoding(file, "utf-8");
    console.log("Contenu du fichier :", text.slice(0, 500)); // affiche les 500 premiers caractères

    const imported = parseCSVText(text);
    if (imported.length === 0) {
      alert("CSV invalide ou vide.");
      return;
    }

    allStudents.value = imported.map((s, idx) => ({ ...s, id: idx + 1 }));
    saveToStorage();
    if (fileInput.value) fileInput.value.value = "";

  } catch (err) {
    console.error("Erreur lors de l'importation du fichier :", err);
    alert("Erreur lors de l'importation du fichier.");
  }
}


function onUploadJustif(event, id) {
  const file = event.target.files[0];
  if (!file) return;

  if (file.type !== "image/png") {
    alert("Le justificatif doit être un fichier PNG.");
    return;
  }

  const reader = new FileReader();
  reader.onload = () => {
    const base64 = reader.result;
    const stu = allStudents.value.find(s => s.id === id);
    if (stu) {
      stu.justificatif = base64;
      stu.absent = false;
      saveToStorage();
    }
  };
  reader.readAsDataURL(file);
}

function viewJustif(id) {
  const stu = allStudents.value.find(s => s.id === id);
  if (stu && stu.justificatif) {
    const win = window.open();
    win.document.write(`<img src="${stu.justificatif}" style="max-width:100%">`);
  }
}
</script>

<style scoped>
.logo {
  width: 60px;
  border-radius: 10px;
  margin: 10px;
}

.navbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  background-color: #791919;
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
}

.navbar ul li a:hover {
  color: #ffc2c2;
}

.student-list-container {
  width: 90%;
  max-width: 1200px;
  margin: 40px auto;
  color: #791919;
}

h1 {
  text-align: center;
  font-weight: 700;
  font-size: 2.5rem;
  margin-bottom: 20px;
}

/* Conteneur des filtres */
.filters-container {
  display: flex;
  justify-content: center;
  flex-wrap: wrap;
  gap: 20px;
  margin-bottom: 30px;
}

select {
  background-color: #fff5f5;
  border-radius: 20px;
  padding: 12px 18px;
  font-family: inherit;
  color: #791919;
  font-size: 14px;
  min-width: 220px;
  cursor: pointer;
  border: none;
  transition: background-color 0.3s ease, box-shadow 0.3s ease;
}

select option[value=""] {
  color: #a9a9a9;
}

select:focus {
  background-color: #ffffff;
  border-color: #ffd6d6;
  outline: none;
  box-shadow: 0 0 0 3px rgba(255, 200, 200, 0.4);
}

.student-list {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.student-list-header {
  display: grid;
  grid-template-columns: 2.5fr 1fr 1fr 1.5fr;
  padding: 10px 20px;
  font-weight: 700;
  border-bottom: 2px solid #ffdfdf;
}

.student-item {
  display: grid;
  grid-template-columns: 2.5fr 1fr 1fr 1.5fr;
  background: #fff5f5;
  border-radius: 15px;
  padding: 15px;
  box-shadow: 0 2px 4px rgba(255, 157, 157, 0.3);
  align-items: center;
}

.actions-cell {
  display: flex;
  gap: 10px;
  justify-content: flex-end;
  align-items: center;
}

.actions-cell input[type="file"] {
  cursor: pointer;
}

.btn-modify {
  background: #791919;
  color: white;
  border: none;
  border-radius: 10px;
  padding: 8px 14px;
  cursor: pointer;
  transition: background-color 0.3s ease;
}

.btn-modify:hover {
  background-color: #a02222;
}

.no-results {
  text-align: center;
  color: #777;
  padding: 20px;
  font-style: italic;
}

</style>