<template>
<div class="navbar">
  <img src="../src\assets\vue.svg" alt="Logo" class="logo" />
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
        <h1 id="titre_emploie_du_temps">Emploi du temps</h1>
        <img src="../src\assets\vue.svg" alt="emploi du temps" id="logo_edt" />
    </div>
    <div class="Informations_supplementaires">
      <div id="Commentaires">
          <h2>Remarques</h2>
          <textarea id="commentaires" ></textarea>
          <button id="BtnCommentaires" @click="envoieCommentaire">Envoyer</button>
          <p>{{ Commentaires }}</p>
      </div>

      <div id="Absences">
          <h2>Absences</h2>
          <textarea id="absences"></textarea>
          <button id="BtnAbsences" @click="envoieAbsences">Envoyer</button>
          <p>{{ absences }}</p>
      </div>
      <div id="Mesures_Disciplinaire">
          <h2>Mesures Disciplinaire</h2>
          <textarea id="mesures_disciplinaires"></textarea>
          <button id="BtnMesures_Disciplinaire" @click="envoieMesures_Disciplinaires">Envoyer</button>
          <p>{{ mesures_disciplinaires }}</p>
      </div>
 </div>

<div class="Info_centre">

    <div id="Appel_non_fait">
      <h2>Appel non fait</h2>
      <textarea id="appel_non_fait"></textarea>
      <button id="BtnAppel_non_fait" @click="envoieAppel_non_fait">Envoyer</button>
      <p>{{ appel_non_fait }}</p>
    </div>

    <div id="selection_groupe">
      <h2>Selection Groupe</h2>
      <select>
        <option value="groupe1">G1A</option>
        <option value="groupe2">G1B</option>
        <option value="groupe3">G2A</option>
        <option value="groupe1">G2B</option>
        <option value="groupe2">G3A</option>
        <option value="groupe3">G3B</option>
        <option value="groupe1">G4A</option>
        <option value="groupe2">G4B</option>
        <option value="groupe3">G5A</option>
        <option value="groupe3">G5B</option>
        <option value="groupe3">G6A</option>
        <option value="groupe3">G6B</option>
        <option value="groupe3">G7A</option>
        <option value="groupe3">G7B</option>
      </select>
    </div>  
</div>


</div>


</template>

<script setup>
import { h, ref } from "vue";
import { useRouter } from "vue-router";
const router = useRouter()
const absences = ref('')
const Commentaires = ref('')

const home = () => {
  router.push("/")
}

    const envoieCommentaire = (event) => {
      const textarea = event.target.previousElementSibling;
      textarea.value = "";
      Commentaires.value = "Envoyé !";
    };

    const envoieAbsences = (event) => {
      const textarea = event.target.previousElementSibling;
      textarea.value = "";
      absences.value = "Envoyé !";
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
}
.Informations_supplementaires {
  display: inline;
  float: right;
  color: #791919;
}

#commentaires, #absences{
  width: 300px;
  height: 150px;
  background-color: lightgray;
}

#mesures_disciplinaires {
  width: 300px;
  height: 80px;
  background-color: lightgray;
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
  background-color: lightgray;
}
.EmploiDuTemps {
  float: left;
  color: #791919;
  margin: 20px;
  margin-top: 100px;
  height: 400px;
  width: 600px;
  background-color: lightgray;
}

#titre_emploie_du_temps {
  text-align: center;
}
#logo_edt {
  display: block;
  margin-left: auto;
  margin-right: auto;
  width: 200px;
  height: 200px;
}   
</style>