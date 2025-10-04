<template>
  <div class="navbar">
    <img src="../src/assets/vue.svg" alt="Logo" class="logo" />
    <ul class="nav-links">
      <li @click="home">retourner au menu</li>
      <li>liste des eleves</li>
      <li>ajouter un eleve</li>
      <li>supprimer un eleve</li>
      <li>modifier un eleve</li>
    </ul>
  </div>

  <div class="content">
    <div class="EmploiDuTemps">
      <h1 id="titre_emploie_du_temps">Emploi du temps du {{ selectedGroup }}</h1>
      <img src="../src/assets/edt.png" alt="emploi du temps" id="logo_edt" />
    </div>

    <div class="Informations_supplementaires">
      <div id="selection_groupe">
        <h2>Selection Groupe</h2>
        <select v-model="selectedGroup">
          <option v-for="g in groupes" :key="g" :value="g">{{ g }}</option>
        </select>
      </div>

      <div id="Appel_non_fait">
        <h2>Appels non faits</h2>
        <textarea v-model="appelNonFait"></textarea>
        <button @click="sendCallNotRealised">Envoyer</button>
        <p>{{ messageCallNotRealise }}</p>
      </div>

      <div id="Commentaires">
        <h2>Remarques</h2>
        <textarea v-model="commentaire"></textarea>
        <button @click="sendComments">Envoyer</button>
        <p>{{ messageComment }}</p>
      </div>

      <div id="Absences">
        <h2>Absences</h2>
        <textarea v-model="absence"></textarea>
        <button @click="sendAbsence">Envoyer</button>
        <p>{{ absenceMessage }}</p>
      </div>

      <div id="Mesures_Disciplinaire">
        <h2>Mesures Disciplinaire</h2>
        <textarea v-model="disciplinary"></textarea>
        <button @click="sendDisciplinary">Envoyer</button>
        <p>{{ disciplinaryMessage }}</p>
      </div>
    </div>
  </div>
</template>

<script setup>
  import { ref } from "vue";
  import { useRouter } from "vue-router";

  const router = useRouter();

  const selectedGroup = ref("G1A");
  const groupes = ["G1A", "G1B", "G2A", "G2B", "G3A", "G3B", "G4A", "G4B", "G5A", "G5B", "G6A", "G6B", "G7A", "G7B"];

  const CallNotRealise = ref("");
  const messageCallNotRealise = ref("");

  const commentaire = ref("");
  const messageComment = ref("");

  const absence = ref("");
  const absenceMessage = ref("");

  const disciplinary = ref("");
  const disciplinaryMessage = ref("");

  const home = () => {
    router.push("/");
  };

  const sendCallNotRealised = () => {
    CallNotRealise.value = "";
    messageCallNotRealise.value = "Envoyé !";
  };

  const sendComments = () => {
    commentaire.value = "";
    messageComment.value = "Envoyé !";
  };

  const sendAbsence = () => {
    absence.value = "";
    absenceMessage.value = "Envoyé !";
  };

  const sendDisciplinary = () => {
    disciplinary.value = "";
    disciplinary.value = "Envoyé !";
  };
</script>

<style>
.navbar {
  display: flex;
  gap: 20px;
  align-items: center;
  background-color: #791919;
  padding: 10px;
}
li {
  list-style: none;
  color: white;
  font-weight: bold;
  display: inline;
  padding: 0 10px;
  cursor: pointer;
}

.Informations_supplementaires {
  display: inline;
  float: right;
  color: #791919;
}

#commentaires, #absences {
  width: 300px;
  height: 150px;
}

#mesures_disciplinaires {
  width: 300px;
  height: 80px;
}

.Info_centre {
  display: inline;
  float: center; 
  color: #791919;
  margin-top: 20px;
}

#appel_non_fait {
  width: 300px;
  height: 80px;
}

.EmploiDuTemps {
  float: left;
  color: #791919;
  margin: 20px;
  margin-top: 100px;
  height: 400px;
  width: 600px;
}

#titre_emploie_du_temps {
  text-align: center;
}
#logo_edt {
  display: block;
  margin-left: auto; 
  margin-right: auto;
  width: 550px;
  height: 500px;
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

#commentaires, #absences {
  height: 150px;
}
#mesures_disciplinaires,
#appel_non_fait {
  height: 80px; 
}

.content {
  display: flex;
  padding: 20px;
  gap: 30px;
}

button {
  background: #791919;
  color: white;
  border: none;
  border-radius: 6px;
  padding: 6px 12px;
  cursor: pointer;
  transition: background 0.2s;
}
button:hover {
  background: #5a1313;
}
</style>
