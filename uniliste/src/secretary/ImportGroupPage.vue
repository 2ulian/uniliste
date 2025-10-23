<template>

    <head class="navbar">
        <img src="../assets/unilim.png" alt="Logo" class="logo" />
        <ul>
            <router-link to="/first" id="nav-links">Menu</router-link>
            <router-link to="/annee" id="nav-links">Annee Universitaire</router-link>
            <router-link to="/student" id="nav-links">Importer étudiant</router-link>
            <router-link to="/ressources" id="nav-links">Importer ressources</router-link>
            <router-link to="/professor" id="nav-links">Importer professeur</router-link>
            <router-link to="/justification" id="nav-links">Saisir justificatif</router-link>
        </ul>
    </head>

    <body>
        <div id="importation">
            <p>importer un fichier de groupes</p>
            <button for="fileInput" class="custom-file-upload" @click="openExplorer">
                📂 Choisir un fichier CSV
            </button>
            <input type="file" @change="importation" accept=".csv" hidden />
        </div>
    </body>
</template>

<script>
export default {
  name: "JustificationImport",
  data() {
    return {
      liste: JSON.parse(localStorage.getItem("justificationList") || "[]")
    };
  },
  methods: {
    openExplorer() {
      this.$el.querySelector('input[type="file"]').click();
    },
    importation(event) {
      const file = event.target.files[0];
      if (!file) return;

      const reader = new FileReader();
      reader.onload = (e) => {
        const contents = e.target.result;
        const lignes = contents.split(/\r?\n/).filter(l => l.trim() !== "");
        this.liste = lignes;
        localStorage.setItem("justificationList", JSON.stringify(this.liste));
      };
      reader.readAsText(file);
    }
  }
};
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
#importation {
  margin: 20px;
  padding: 10px;
  color: #791919;
  background-color: #fff5f5;
  box-shadow: 0 4px 8px rgba(255, 157, 157, 0.3);
  border-radius: 1rem;
  width: 500px;
  height: 400px;
}
input[type="file"] {
  margin-top: 10px;
  margin-bottom: 20px;
  color: #791919;
}
</style>
