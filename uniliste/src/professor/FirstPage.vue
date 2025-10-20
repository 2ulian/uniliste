<template>
  <!-- Importation de la font Plus Jakarta Sans-->
  <head>
  <link rel="preconnect" href="https://fonts.googleapis.com">
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
  <link href="https://fonts.googleapis.com/css2?family=Plus+Jakarta+Sans:ital,wght@0,200..800;1,200..800&display=swap" rel="stylesheet">
  </head>

  <div>
    <nav class="navbar">
      <img src="../assets/unilim.png" alt="Logo" class="logo" />
      <ul>
        <li><router-link to="/menu">retourner au menu</router-link></li>
        <li><router-link to="/liste-eleves">liste des eleves</router-link></li>
        <li><router-link to="/ajouter-eleve">ajouter un eleve</router-link></li>
        <li><router-link to="/supprimer-eleve">supprimer un eleve</router-link></li>
        <li><router-link to="/modifier-eleve">modifier un eleve</router-link></li>
      </ul>
    </nav>

    <main class="grid-container">
      <section class="grid-item">
        <h1>Bonjour &lt;Professeur&gt;</h1>
        <h2>Voici vos prochains cours</h2>
        <div class="info-box">
          <div class="info-item"><span>CM Amp C</span> <span>R1.01</span> <span>10h30–12h00</span></div>
          <div class="info-item"><span>TD 209</span> <span>R1.01</span> <span>13h30–15h30</span></div>
        </div>
      </section>

      <section class="grid-item">
        <h2>Mesures disciplinaires</h2>
        <div class="info-box large"></div>
      </section>

      <section class="grid-item">
        <h2>Commentaires</h2>
        <textarea v-model="comment" placeholder="Entrez votre commentaire ici..."></textarea>
        <button @click="sendComment">Envoyer</button>
        <p>{{ commentMessage }}</p>
      </section>

      <section class="grid-item">
        <h2>Appels à faire</h2>
        <div class="info-box">
          <div class="info-item">TP  R3.02  10h30–12h30</div>
          <div class="info-item">TD  R1.01  08h30–10h30</div>
          <div class="info-item">CM  R2.01  13h30–15h00</div>
        </div>
      </section>

      <section class="grid-item">
        <h2>Absence des élèves</h2>
        <div class="info-box">
          <div class="info-item"></div>
          <div class="info-item"></div>
          <div class="info-item"></div>
        </div>
      </section>

      <section class="grid-item">
        <h2>Absence des personnels</h2>
        <div class="info-box">
          <div class="info-item"></div>
          <div class="info-item"></div>
          <div class="info-item"></div>
        </div>
      </section>
    </main>
  </div>
</template>

<script setup>
import { ref } from "vue";

const comment = ref("");
const commentMessage = ref("");

const sendComment = () => {
  commentMessage.value = "Envoyé !";
  setTimeout(() => (commentMessage.value = ""), 1000);
};
</script>

<style>
body {
  font-family: 'Plus Jakarta Sans', sans-serif;
}

.logo {
  width: 60px;
  margin: 10px;
}

.navbar {
  display: flex;
  align-items: center;
  background-color: #791919;
  padding: 10px;
}

.navbar ul {
  list-style: none;
  display: flex;
  margin: 0;
  gap: 15px;
}

.navbar li a {
  color: white;
  font-weight: bold;
  text-decoration: none;
}

.grid-container {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  grid-template-rows: repeat(2, auto);
  gap: 30px;
  padding: 40px;
  color: #791919;
}

.grid-item {
  background-color: white;
  border-radius: 15px;
  padding: 20px;
  box-shadow: 0 2px 6px rgba(255, 255, 255, 0);
}

.info-box {
  width: 400px;
  height: 150px;
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.info-item {
  background-color: #fff5f5;
  border-radius: 15px;
  padding: 10px 15px;
  box-shadow: 0 2px 4px rgba(255, 157, 157, 0.3);
  font-size : 14px;
  min-height: 19px; 
}

.large {
  height: 150px;
}

textarea {
background-color: #fff5f5;
  width : 400px;
  height : 150px; 
  border-radius: 15px;
  padding: 10px 15px;
  box-shadow: 0 2px 4px rgba(255, 157, 157, 0.3);
  resize: none;
  border: none;
  font-family : inherit;
}

textarea::placeholder {
  font-style: italic;
  color: #791919;
}

button {
  background-color: #791919;
  color: white;
  border: none;
  border-radius: 10px;
  padding: 8px 16px;
  margin-top: 10px;
  cursor: pointer;
}

button:hover {
  background-color: #5a1212;
}
</style>
