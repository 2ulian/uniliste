<template>

    <head class="navbar">
        <img src="../assets/unilim.png" alt="Logo" class="logo" />
        <ul>
            <router-link to="/first" id="nav-links">Menu</router-link>
            <router-link to="/cours" id="nav-links">Importer étudiant</router-link>
            <router-link to="/ressources" id="nav-links">Importer groupes</router-link>
            <router-link to="/ressources" id="nav-links">Importer ressources</router-link>
            <router-link to="/ressources" id="nav-links">Importer professeur</router-link>
            <router-link to="/ressources" id="nav-links">Saisir justificatif</router-link>
            <router-link to="/about" id="nav-links">A Propos</router-link>
        </ul>
    </head>

    <body>
        <div>
            <p>importer un fichier</p>
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
<style scoped></style>
