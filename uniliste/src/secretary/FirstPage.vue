<template>
  <head class="navbar">
    <img src="../assets/unilim.png" alt="Logo" class="logo" />
    <ul>
      <router-link to="/annee" id="nav-links">Annee Universitaire</router-link>
      <router-link to="/student" id="nav-links">Importer étudiant</router-link>
      <router-link to="/group" id="nav-links">Importer groupes</router-link>
      <router-link to="/ressources" id="nav-links">Importer ressources</router-link>
      <router-link to="/professor" id="nav-links">Importer professeur</router-link>
      <router-link to="/justification" id="nav-links">Saisir justificatif</router-link>
    </ul>
  </head>

  <div class="content">
    <div class="Informations_supplementaires">
      <div id="Commentaires">
        <h2>Remarques</h2>
        <textarea v-model="commentaire"></textarea>
        <div id="ligne">
          <button @click="sendComments" class="Send-button">Envoyer</button>
          <p>{{ commentMessage }}</p>
        </div>

      </div>

      <div id="Absences">
        <h2>Absences</h2>
        <textarea v-model="absence"></textarea>
        <div id="ligne">
          <button @click="sendAbsence" class="Send-button">Envoyer</button>
          <p>{{ absenceMessage }}</p>
        </div>

      </div>

      <div id="Mesures_Disciplinaire">
        <h2>Mesures Disciplinaires</h2>
        <textarea v-model="disciplinary"></textarea>
        <div id="ligne">
          <button @click="sendDisciplinary" class="Send-button">Envoyer</button>
          <p>{{ disciplinaryMessage }}</p>
        </div>
      </div>
    </div>
    <div class="Etudiants_absents">
      <h2>Etudiants absents</h2>
      <area>

      </area>
    </div>
  </div>
</template>

<script setup>
import { ref } from "vue";

const selectedGroup = ref("G1A");

const commentaire = ref("");
const commentMessage = ref("");

const absence = ref("");
const absenceMessage = ref("");

const disciplinary = ref("");
const disciplinaryMessage = ref("");

const sendComments = () => {
  commentaire.value = "";
  commentMessage.value = "Envoyé !";
  setTimeout(() => {
    commentMessage.value = "";
  }, 500);
};

const sendAbsence = () => {
  absence.value = "";
  absenceMessage.value = "Envoyé !";
  setTimeout(() => {
    absenceMessage.value = "";
  }, 500);
};

const sendDisciplinary = () => {
  disciplinary.value = "";
  disciplinaryMessage.value = "Envoyé !";
  setTimeout(() => {
    disciplinaryMessage.value = "";
  }, 500);
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

.content {
  display: flex;
  padding: 20px;
  gap: 30px;
}

.Informations_supplementaires {
  display: inline;
  float: right;
  color: #791919;
}

#commentaires,
#absences {
  height: 150px;
}

#mesures_disciplinaires {
  height: 80px;
}

#ligne {
  display: flex;
  align-items: center;
  gap: 10px;
}

textarea {
  width: 100%;
  height: 80px;
  background-color: #fff5f5;
  box-shadow: 0 2px 4px rgba(255, 157, 157, 0.3);
  padding: 8px;
  margin-bottom: 5px;
  resize: none;
  box-sizing: border-box;
}

.Send-button {
  background: #791919;
  color: white;
  border: none;
  border-radius: 6px;
  padding: 6px 12px;
  cursor: pointer;
  box-shadow: 0 2px 4px rgba(255, 157, 157, 0.3);
  overflow: hidden;
}

.Etudiants_absents {
  float: right;
  margin: 20px;
  padding: 10px;
  color: #791919;
  background-color: #fff5f5;
  box-shadow: 0 4px 8px rgba(255, 157, 157, 0.3);
  border-radius: 1rem;
  width: 500px;
  height: 400px;
}
</style>
