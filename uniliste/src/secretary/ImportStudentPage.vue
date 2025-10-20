<template>

    <head class="navbar">
        <img src="../assets/unilim.png" alt="Logo" class="logo" />
        <ul>
            <router-link to="/first" id="nav-links">Menu</router-link>
            <router-link to="/annee" id="nav-links">Annee Universitaire</router-link>
            <router-link to="/group" id="nav-links">Importer groupes</router-link>
            <router-link to="/ressources" id="nav-links">Importer ressources</router-link>
            <router-link to="/professor" id="nav-links">Importer professeur</router-link>
            <router-link to="/justification" id="nav-links">Saisir justificatif</router-link>
        </ul>
    </head>

    <body>
        <div>
            <p>importer un fichier d'eleve</p>
            <button @click="importation">Importer</button>
        </div>
    </body>
</template>

<script setup>
import { ref } from "vue";
function importation() {
    var delimitation = ",";
    var line = '\n';
    var i = document.getElementById('file');
    var table = document.getElementById('table');

    if(!i){
        return;
    }
    i.addEventListener('change', function(e){
        if(file && file.length > 0  ){
            return;
        }
       
      
    });
}
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
</style>
