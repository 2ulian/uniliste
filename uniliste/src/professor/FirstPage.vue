<template>
  <div>
    <nav class="navbar">
      <img src="../assets/unilim.png" alt="Logo" class="logo" />
      <ul>
        <li><router-link to="/prof">retourner au menu</router-link></li>
        <li><router-link to="/list">liste des eleves</router-link></li>
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
          <div class="info-item">TP  R3.02  10h30–12h30</div>
          <div class="info-item">TD  R1.01  08h30–10h30</div>
          <div class="info-item">CM  R2.01  13h30–15h00</div>
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
/* Style global de la police */
body {
  font-family: 'Plus Jakarta Sans', sans-serif;
  background-color: #fafafa; /* Léger fond gris pour que les cartes ressortent */
  margin: 0;
}

/* --- STYLES NAVBAR MODERNISÉS --- */
.logo {
  width: 60px;
  height: auto;
  border-radius: 10px;
  margin: 10px 0; /* Marge verticale pour la hauteur de la navbar */
}

.navbar {
  display: flex;
  align-items: center;
  justify-content: space-between; /* Sépare logo et liens */
  background-color: #791919;
  padding: 0 5%; /* Espace sur les côtés */
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15); /* Ombre moderne */
  overflow: hidden;
}

.navbar ul {
  list-style: none;
  display: flex;
  margin: 0;
  padding: 0;
}

.navbar li a {
  color: white;
  font-weight: 600;
  text-decoration: none;
  font-size: 15px;
  text-transform: capitalize;
  padding: 24px 18px;
  display: block;
  position: relative; /* Nécessaire pour l'animation ::after */
  transition: color 0.3s ease;
}

/* Ligne animée */
.navbar li a::after {
  content: '';
  position: absolute;
  bottom: 18px; /* Position de la ligne */
  left: 50%;
  width: 0; /* Largeur de 0 au repos */
  height: 3px;
  background: #ffc2c2;
  transform: translateX(-50%);
  transition: width 0.3s ease-in-out;
}

/* Effet au survol */
.navbar li a:hover {
  color: #ffc2c2;
}

.navbar li a:hover::after {
  width: 70%; /* La ligne grandit au survol */
}

/* Effet sur le lien actif */
.navbar li a.router-link-exact-active {
  color: #ffc2c2;
  font-weight: 700;
}

.navbar li a.router-link-exact-active::after {
  width: 70%; /* La ligne est visible sur la page active */
}

/* --- STYLES DU CONTENU (GRID) --- */

.grid-container {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(400px, 1fr)); /* Responsive */
  grid-template-rows: repeat(2, auto);
  gap: 30px;
  padding: 40px;
  color: #791919;
}

.grid-item {
  background-color: white;
  border-radius: 15px;
  padding: 20px;
  /* Ombre plus subtile pour les cartes */
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.05); 
  display: flex; /* Ajouté pour un meilleur alignement */
  flex-direction: column; /* Ajouté */
}

h1, h2 {
  margin-top: 0; /* Retire la marge haute par défaut */
}

.info-box {
  width: 100%; /* Prend toute la largeur de la carte */
  /* height: 150px; <-- Retiré pour la flexibilité */
  flex-grow: 1; /* Fait grandir la box pour remplir l'espace */
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
  min-height: 1.2em; /* Hauteur minimale pour les items vides */
  display: flex; /* Ajouté pour espacer les <span> */
  justify-content: space-between; /* Ajouté */
  flex-wrap: wrap; /* Ajouté */
}

.large {
  /* height: 150px; <-- Remplacé par flex-grow dans .info-box */
}

textarea {
  background-color: #fff5f5;
  width : 100%; /* Largeur flexible */
  /* height : 150px; <-- Remplacé par flex-grow */
  flex-grow: 1; /* Prend l'espace disponible */
  min-height: 150px; /* Hauteur minimale */
  border-radius: 15px;
  padding: 10px 15px;
  box-shadow: 0 2px 4px rgba(255, 157, 157, 0.3);
  resize: none;
  border: none;
  font-family : inherit;
  box-sizing: border-box; /* Important pour le width: 100% */
}

textarea::placeholder {
  font-style: italic;
  color: #c9a9a9;
}

textarea:focus {
  outline: none;
  box-shadow: 0 0 0 3px rgba(255, 157, 157, 0.6);
}

button {
  background-color: #791919;
  color: white;
  border: none;
  border-radius: 10px;
  padding: 10px 16px;
  margin-top: 10px;
  cursor: pointer;
  font-family: inherit;
  font-weight: 600;
  font-size: 15px;
  transition: background-color 0.2s;
}

button:hover {
  background-color: #5a1212;
}

p {
  margin-top: 10px;
  font-size: 14px;
}
</style>