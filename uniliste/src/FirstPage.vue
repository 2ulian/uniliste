<template>
  <div class="navbar">
    <img src="../src\assets\unilim.png" alt="Logo" class="logo" />
    <ul>
      <router-link to="/groups" id="nav-links">Groupes</router-link>
      <router-link to="/cours" id="nav-links">Cours</router-link>
      <router-link to="/ressources" id="nav-links">Ressources</router-link>
    </ul>
  </div>

  <div class="content">
    <div class="EmploiDuTemps">
      <h1 id="titre_emploie_du_temps">Emploi du temps du {{ selectedGroup }}</h1>
      <img src="../src/assets/edt.png" alt="emploi du temps" id="logo_edt" />
    </div>

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
        <h2>Mesures Disciplinaire</h2>
        <textarea v-model="disciplinary"></textarea>
        <div id="ligne">
          <button @click="sendDisciplinary" class="Send-button">Envoyer</button>
          <p>{{ disciplinaryMessage }}</p>
        </div>
      </div>
    </div>
    <div class="Appel_non_fait">
      <h2>Appels non faits</h2>
      <area></area>
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

<style>
.logo {
  width: 70px;
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

.EmploiDuTemps {
  text-align: center;
  float: left;
  color: #791919;
  margin: 20px;
  margin-top: 5px;
  width: 600px;
}

#logo_edt {
  display: block;
  margin-left: auto;
  margin-right: auto;
  width: 550px;
  height: 450px;
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
  border-radius: 8px;
  border: 1px solid #e0b3b3;
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
  border: 2px solid #05060f;
  box-shadow: 0.2rem 0.2rem #05060f;
  overflow: hidden;
}

.Appel_non_fait {
  float: right;
  margin: 20px;
  padding: 10px;
  color: #791919;
  background-color: #fff5f5;
  border: solid #e0b3b3;
  border-radius: 1rem;
  width: 500px;
  height: 400px;
}
</style>
