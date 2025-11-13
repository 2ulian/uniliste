<template>

    <head class="navbar">
        <img src="../../assets/unilim.png" alt="Logo" class="logo" />
        <ul>
            <li><router-link to="/first" id="nav-links">Menu</router-link></li>
            <li><router-link to="/annee" id="nav-links">Annee Universitaire</router-link></li>
            <li><router-link to="/student" id="nav-links">Importer étudiant</router-link></li>
            <li><router-link to="/group" id="nav-links">Importer groupes</router-link></li>
            <li><router-link to="/ressources" id="nav-links">Importer ressources</router-link></li>
            <li><router-link to="/justification" id="nav-links">Saisir justificatif</router-link></li>
            <li><router-link to="/" id="nav-links">Quitter</router-link></li>
        </ul>
    </head>

    <body>
        <div id="importation">
            <p>importer un fichier de professeur</p>
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
  padding: 0 5%;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
  overflow: hidden;
}

.navbar ul {
  list-style: none;
  display: flex;
  margin: 0;
  padding: 0;
}

.navbar li a {
  color: white;
  font-weight: 600;
  text-decoration: none;
  font-size: 15px;
  text-transform: capitalize;
  padding: 24px 18px;
  display: block;
  position: relative;
  transition: color 0.3s ease;
}

.navbar li a::after {
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

.navbar li a:hover {
  color: #ffc2c2;
}

.navbar li a:hover::after {
  width: 70%;
}

.navbar li a.router-link-exact-active {
  color: #ffc2c2;
  font-weight: 700;
}

.navbar li a.router-link-exact-active::after {
  width: 70%;
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
