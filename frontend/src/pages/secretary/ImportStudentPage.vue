<template>
  <nav class="navbar">
    <img src="../assets/unilim.png" alt="Logo" class="logo" />
    <ul>
      <li><router-link to="/first" id="nav-links">Menu</router-link></li>
      <li><router-link to="/annee" id="nav-links">Annee Universitaire</router-link></li>
      <li><router-link to="/group" id="nav-links">Importer groupes</router-link></li>
      <li><router-link to="/ressources" id="nav-links">Importer ressources</router-link></li>
      <li><router-link to="/professor" id="nav-links">Importer professeur</router-link></li>
      <li><router-link to="/justification" id="nav-links">Saisir justificatif</router-link></li>
      <li><router-link to="/" id="nav-links">Quitter</router-link></li>
    </ul>
  </nav>

  <main class="student-list-container">
    <div style="display:flex; justify-content:space-between; align-items:center; gap:16px; flex-wrap:wrap;">
      <h1>Liste des élèves</h1>
      <div>
        <button
          class="custom-file-upload btn-modify"
          @click="openExplorer"
          style="background:#791919; color:white;"
        >
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
      </div>

      <div v-for="student in filteredStudents" :key="student.id" class="student-item">
        <span>{{ student.name }}</span>
        <span>{{ student.promo }}</span>
        <span>{{ student.group }}</span>
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

// Chargement localStorage au montage
onMounted(() => {
  const raw = localStorage.getItem(STORAGE_KEY);
  if (raw) {
    try {
      const parsed = JSON.parse(raw);
      if (Array.isArray(parsed) && parsed.length) {
        allStudents.value = parsed;
      }
    } catch (e) {
      console.warn("Erreur parsing localStorage:", e);
    }
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

const splitSemicolonAware = (line) => {
  return line.split(/;(?=(?:[^"]*"[^"]*")*[^"]*$)/);
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
// Split en , en ignorant les , dans guillemets
const splitCommaAware = (line) => {
  return line.split(/,(?=(?:[^"]*"[^"]*")*[^"]*$)/);
};

function parseCSVText(text) {
  const lines = text
    .split(/\r?\n/)
    .map((l) => l.trim())
    .filter((l) => l !== "");
  if (lines.length === 0) return [];

  const headerLine = lines[0];
  const headerCols = splitCommaAware(headerLine); // <-- ici
  const mapIdx = mapHeaderIndex(headerCols);

  const rows = [];
  for (let i = 1; i < lines.length; i++) {
    const cols = splitCommaAware(lines[i]); // <-- ici
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
      mapIdx.groupe !== -1 ? mapIdx.groupe : mapIdx.group !== -1 ? mapIdx.group : -1;
    const group = groupeIdx !== -1 ? normalizeCell(cols[groupeIdx] || "") : "";

    if (name) {
      rows.push({
        name,
        promo,
        group,
      });
    }
  }

  return rows.map((s, idx) => ({ ...s, id: idx + 1 }));
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
      } catch {
        // ignore
      }
    }
    if (text.includes("\u0000")) {
      try {
        text = await readFileAsTextWithEncoding(file, "utf-16");
      } catch {
        // ignore
      }
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
    console.error("Erreur lecture fichier :", err);
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
  border-radius: 12px;
  margin: 12px 10px;
  filter: drop-shadow(1px 1px 2px rgba(0, 0, 0, 0.2));
}

.navbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  background-color: #7a1919;
  font-family: 'Plus Jakarta Sans', sans-serif;
  padding: 0 5%;
  box-shadow: 0 3px 10px rgba(120, 0, 0, 0.3);
  position: sticky;
  top: 0;
  z-index: 1000;
}

.navbar ul {
  display: flex;
  list-style: none;
  padding: 0;
  margin: 0;
}

.navbar ul li a {
  color: #fff;
  padding: 22px 20px;
  font-weight: 600;
  font-size: 15px;
  text-transform: capitalize;
  text-decoration: none;
  display: block;
  position: relative;
  transition: color 0.3s ease;
  border-radius: 6px;
}

.navbar ul li a::after {
  content: '';
  position: absolute;
  bottom: 16px;
  left: 50%;
  width: 0;
  height: 3px;
  background: #ffc2c2;
  transform: translateX(-50%);
  transition: width 0.3s ease-in-out;
  border-radius: 2px;
}

.navbar ul li a:hover {
  color: #ffc2c2;
  background-color: rgba(255, 194, 194, 0.15);
}

.navbar ul li a:hover::after,
.navbar ul li a.router-link-exact-active::after {
  width: 70%;
}

.navbar ul li a.router-link-exact-active {
  color: #ffc2c2;
  font-weight: 700;
  background-color: rgba(255, 194, 194, 0.25);
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
  font-size: 2.8rem;
  margin-bottom: 25px;
  letter-spacing: 1px;
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
  border-radius: 25px;
  padding: 14px 22px;
  font-family: inherit;
  color: #791919;
  font-size: 15px;
  min-width: 280px;
  cursor: pointer;
  border: 2px solid transparent;
  transition: border-color 0.3s ease;
}

select option[value=""] {
  color: #a9a9a9;
}

select:focus {
  background-color: #ffffff;
  border-color: #ff7b7b;
  outline: none;
  box-shadow: 0 0 6px 3px rgba(255, 123, 123, 0.3);
}

.student-list {
  display: flex;
  flex-direction: column;
  gap: 14px;
}

.student-list-header {
  display: grid;
  grid-template-columns: 2.5fr 1fr 1fr 1.5fr;
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
  grid-template-columns: 2.5fr 1fr 1fr 1.5fr;
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
  display: flex;
  gap: 12px;
  justify-content: flex-end;
}

.btn-modify,
.btn-delete {
  border: none;
  border-radius: 12px;
  padding: 9px 18px;
  color: white;
  cursor: pointer;
  font-weight: 700;
  font-size: 14px;
  transition: background-color 0.3s ease, opacity 0.2s ease;
  user-select: none;
}

.btn-modify {
  background-color: #7a1919;
}

.btn-delete {
  background-color: #d93030;
}

.btn-modify:hover {
  background-color: #5e1212;
}

.btn-delete:hover {
  background-color: #b22222;
}

.custom-file-upload {
  border: none;
  padding: 12px 20px;
  border-radius: 14px;
  cursor: pointer;
  font-weight: 700;
  font-size: 15px;
  box-shadow: 0 4px 10px rgba(0, 0, 0, 0.1);
  background-color: #791919;
  color: white;
  transition: background-color 0.3s ease;
  display: inline-flex;
  align-items: center;
  gap: 8px;
}

.custom-file-upload:hover {
  background-color: #5c1212;
}

.no-results {
  text-align: center;
  color: #777;
  padding: 24px;
  font-style: italic;
  font-size: 16px;
}
</style>