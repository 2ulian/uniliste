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
      <div id="Remarques">
          <h2>Remarques</h2>
          <textarea name="remarques" id="remarques"></textarea>
      </div>

      <div id="Absences">
          <h2>Absences</h2>
          <textarea name="absences" id="absences"></textarea>
      </div>

    </div>


</div>


</template>

<script>
import { h, ref } from "vue";
import { useRouter } from "vue-router";
const router = useRouter()
const home = () => {
  router.push("/")
}


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
  float: center;
  color: #791919;
}

#remarques, #absences {
  width: 300px;
  height: 200px;
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