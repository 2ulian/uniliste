<template>
  <nav class="navbar">
    <img src="../../assets/unilim.png" alt="Logo" class="logo" />
    <ul>
      <li><router-link to="/first" class="nav-links">Menu</router-link></li>
      <li><router-link to="/annee" class="nav-links">Annee Universitaire</router-link></li>
      <li><router-link to="/group" class="nav-links">Importer groupes</router-link></li>
      <li><router-link to="/ressources" class="nav-links">Importer ressources</router-link></li>
      <li><router-link to="/professor" class="nav-links">Importer professeur</router-link></li>
      <li><router-link to="/justification" class="nav-links">Saisir justificatif</router-link></li>
      <li><router-link to="/" class="nav-links">Quitter</router-link></li>
    </ul>
  </nav>

  <main class="student-list-container">
    <div class="header-bar">
      <h1>Liste des élèves</h1>
      <div>
        <button class="custom-file-upload btn-modify import-btn" @click="openExplorer">
          📂 Importer un fichier CSV
        </button>
        <input ref="fileInput" type="file" @change="onFileChange" accept=".csv" hidden />
      </div>
    </div>

    <div class="student-list">
      <div class="student-list-header">
        <span>Nom / Prénom</span>
        <span>Promo</span>
        <span>Groupe</span>
        <span>Aménagement</span>
        <span>Action</span>
      </div>

      <div v-for="student in filteredStudents" :key="student.id" class="student-item">
        <span>{{ student.name }}</span>
        <span>{{ student.promo }}</span>
        <span>{{ student.group }}</span>
        <span>{{ student.aménagement }}</span>
        <div class="actions-cell">
          <button class="btn-modify" @click="modifyStudent(student.id)">Modifier</button>
          <button class="btn-delete" @click="deleteStudent(student.id)">Supprimer</button>
        </div>
      </div>

      <p v-if="filteredStudents.length === 0" class="no-results">
        Aucun élève ne correspond aux filtres sélectionnés.
      </p>
    </div>
  </main>
</template>

<script setup>
import { ref, computed, onMounted } from "vue";
import { useRouter } from "vue-router";

const router = useRouter();

const fileInput = ref(null);
const selectedPromo = ref("");
const selectedTD = ref("");
const selectedTP = ref("");
const STORAGE_KEY = "studentsList_v1";

const allStudents = ref([]);

onMounted(() => {
  const raw = localStorage.getItem(STORAGE_KEY);
  if (raw) {
    try {
      const parsed = JSON.parse(raw);
      if (Array.isArray(parsed) && parsed.length) {
        allStudents.value = parsed;
      }
    } catch (e) {}
  }
});

const filteredStudents = computed(() => {
  let students = allStudents.value.slice();
  if (selectedPromo.value) {
    students = students.filter((s) => s.promo === selectedPromo.value);
  }
  if (selectedTD.value) {
    students = students.filter((s) => s.group.startsWith(selectedTD.value));
  }
  if (selectedTP.value) {
    students = students.filter((s) => s.group === selectedTP.value);
  }
  return students;
});

const openExplorer = () => {
  if (fileInput.value) fileInput.value.click();
};

function saveToStorage() {
  localStorage.setItem(STORAGE_KEY, JSON.stringify(allStudents.value));
}

function normalizeCell(cell) {
  if (typeof cell !== "string") return "";
  let c = cell.trim();
  if (
    (c.startsWith('"') && c.endsWith('"')) ||
    (c.startsWith("'") && c.endsWith("'"))
  ) {
    c = c.slice(1, -1);
  }
  return c.trim();
}

const splitCommaAware = (line) => {
  return line.split(/,(?=(?:[^"]*"[^"]*")*[^"]*$)/);
};

function mapHeaderIndex(headerArr) {
  const norm = (s) =>
    s
      .toLowerCase()
      .normalize("NFD")
      .replace(/[\u0300-\u036f]/g, "")
      .replace(/\s+/g, "");
  const h = headerArr.map((hd) => norm(normalizeCell(hd)));
  return {
    nom: h.indexOf("nom"),
    prenom: h.indexOf("prenom"),
    nomcomplet: h.indexOf("nomcomplet"),
    name: h.indexOf("name"),
    promo: h.indexOf("promo"),
    groupe: h.indexOf("groupe"),
    group: h.indexOf("group"),
  };
}

function parseCSVText(text) {
  const lines = text
    .split(/\r?\n/)
    .map((l) => l.trim())
    .filter((l) => l !== "");
  if (lines.length === 0) return [];
  const headerLine = lines[0];
  const headerCols = splitCommaAware(headerLine);
  const mapIdx = mapHeaderIndex(headerCols);
  const rows = [];

  for (let i = 1; i < lines.length; i++) {
    const cols = splitCommaAware(lines[i]);
    let name = "";
    const hasNomPrenom = mapIdx.nom !== -1 && mapIdx.prenom !== -1;
    const hasNomComplet = mapIdx.nomcomplet !== -1;
    const hasName = mapIdx.name !== -1;

    if (hasNomPrenom) {
      const nom = normalizeCell(cols[mapIdx.nom] || "");
      const prenom = normalizeCell(cols[mapIdx.prenom] || "");
      name = (nom + " " + prenom).trim();
    } else if (hasNomComplet) {
      name = normalizeCell(cols[mapIdx.nomcomplet] || "");
    } else if (hasName) {
      name = normalizeCell(cols[mapIdx.name] || "");
    } else {
      name = normalizeCell(cols[0] || "");
    }

    const promoIdx = mapIdx.promo;
    const promo = promoIdx !== -1 ? normalizeCell(cols[promoIdx] || "") : "";

    const groupeIdx =
      mapIdx.groupe !== -1
        ? mapIdx.groupe
        : mapIdx.group !== -1
        ? mapIdx.group
        : -1;

    const group = groupeIdx !== -1 ? normalizeCell(cols[groupeIdx] || "") : "";

    if (name) {
      rows.push({
        name,
        promo,
        group,
      });
    }
  }

  return rows.map((s, idx) => ({
    ...s,
    id: idx + 1,
  }));
}

function readFileAsTextWithEncoding(file, encoding = "utf-8") {
  return new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = (e) => resolve(e.target.result);
    reader.onerror = (e) => reject(e);
    try {
      reader.readAsText(file, encoding);
    } catch (err) {
      reject(err);
    }
  });
}

async function onFileChange(event) {
  const file = event.target.files ? event.target.files[0] : event;
  if (!file) return;

  try {
    let text = await readFileAsTextWithEncoding(file, "utf-8");

    const looksBinary = text.includes("\u0000");
    if (looksBinary) {
      try {
        text = await readFileAsTextWithEncoding(file, "utf-16le");
      } catch {}
    }
    if (text.includes("\u0000")) {
      try {
        text = await readFileAsTextWithEncoding(file, "utf-16");
      } catch {}
    }

    const imported = parseCSVText(text);

    if (imported.length === 0) {
      alert(
        "Aucune ligne trouvée dans le CSV ou format non reconnu. Vérifie que le CSV contient l'entête : Nom;Prénom;Promo;Groupe"
      );
      return;
    }

    allStudents.value = imported;
    saveToStorage();

    if (fileInput.value) {
      fileInput.value.value = "";
    }
  } catch (err) {
    alert(
      "Erreur lors de la lecture du fichier. Vérifie l'encodage et le format (Format attendu : séparateur ;)"
    );
  }
}

const modifyStudent = (id) => {
  console.log(`Demande de modification pour l'élève ${id}`);
};

const deleteStudent = (id) => {
  if (!confirm("Supprimer cet élève ?")) return;
  allStudents.value = allStudents.value
    .filter((s) => s.id !== id)
    .map((s, idx) => ({ ...s, id: idx + 1 }));
  saveToStorage();
};

const home = () => {
  router.push("/first");
};
</script>

<style scoped>
.logo {
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

.student-list-header {
  display: grid;
  grid-template-columns: 2.5fr 1fr 1fr 1.5fr 1.5fr;
  gap: 15px;
  padding: 10px 20px;
  font-weight: 700;
  font-size: 14px;
  color: #555;
  border-bottom: 2px solid #ffdfdf;
  background-color: #ffe5e5;
  border-radius: 12px;
}

.student-item {
  display: grid;
  grid-template-columns: 2.5fr 1fr 1fr 1.5fr 1.5fr;
  gap: 15px;
  align-items: center;
  background-color: #fff5f5;
  border-radius: 15px;
  padding: 15px 20px;
  box-shadow: 0 2px 4px rgba(255, 130, 130, 0.35);
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
  background-color: #d93030;
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
