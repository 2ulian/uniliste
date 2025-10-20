<template>
    <div class="page">
        <header class="navbar">
            <img src="../assets/unilim.png" alt="Logo" class="logo" />
            <ul>
                <router-link to="/first" id="nav-links">Menu</router-link>
                <router-link to="/annee" id="nav-links">Année Universitaire</router-link>
                <router-link to="/student" id="nav-links">Importer étudiant</router-link>
                <router-link to="/group" id="nav-links">Importer groupes</router-link>
                <router-link to="/ressources" id="nav-links">Importer ressources</router-link>
                <router-link to="/professor" id="nav-links">Importer professeur</router-link>
            </ul>
        </header>

        <main>
            <div id="student_abs">
                <p>Importer un fichier de justification</p><input type="file" @change="importation" accept=".csv" />
                <ul>
                    <h2>Choisir l'étudiant absent</h2>
                    <li v-for="(ligne, i) in liste" :key="i">{{ ligne }}</li>
                </ul>
            </div>


        </main>
    </div>
</template>

<script>
export default {
    name: "JustificationImport",
    data() {
        return {
            liste: []
        };
    },
    methods: {
        importation(event) {
            const file = event.target.files[0];
            if (file) {
                const reader = new FileReader();
                reader.onload = (e) => {
                    const contents = e.target.result;
                    const ligne = contents.split(/\r?\n/).filter(l => l.trim() !== "");

                    this.liste.push(...ligne);  
                };
                reader.readAsText(file);
                
            }
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
    color: white;
}

#nav-links {
    color: white;
    padding: 20px;
    text-decoration: none;
    font-weight: bold;
}

#student_abs {
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

li {
    list-style-type: none;
    margin: 5px 0;
}
</style>
