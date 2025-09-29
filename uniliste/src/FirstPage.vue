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
        <h1>Emploi du temps</h1>
    </div>
    <div class="Remarques">
        <h2>Remarques</h2>
        <textarea name="remarques" id="remarques"></textarea>
    </div>

    <div class="Absences">
        <h2>Absences</h2>
        <textarea name="absences" id="absences"></textarea>
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
.Remarques, .Absences {
  color: #791919;
  float: left;
  width: 200px;
  height: 100px;
  gap : 20px;
}
</style>