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
        <li><router-link to="/">quitter</router-link></li>
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
body {
  font-family: 'Plus Jakarta Sans', sans-serif;
  background-color: #fafafa; 
  margin: 0;
}


.logo {
  width: 60px;
  height: auto; 
  border-radius: 10px;
  margin: 10px ;
}

.navbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  background-color: #791919;
  padding: 0 5%;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
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
  position: relative; 
  transition: color 0.3s ease;
}

.navbar li a::after {
  content: '';
  position: absolute;
  bottom: 18px; 
  left: 50%;
  width: 0; 
  height: 3px;
  background: #ffc2c2;
  transform: translateX(-50%);
  transition: width 0.3s ease-in-out;
}

.navbar li a:hover {
  color: #ffc2c2;
}

.navbar li a:hover::after {
  width: 70%;
}

.navbar li a.router-link-exact-active {
  color: #ffc2c2;
  font-weight: 700;
}

.navbar li a.router-link-exact-active::after {
  width: 70%; 
}

.grid-container {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(400px, 1fr));
  grid-template-rows: repeat(2, auto);
  gap: 30px;
  padding: 40px;
  color: #791919;
}

.grid-item {
  background-color: white;
  border-radius: 15px;
  padding: 20px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.05); 
  display: flex;
  flex-direction: column;
}

h1, h2 {
  margin-top: 0;
}

.info-box {
  width: 100%;
  flex-grow: 1;
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
  min-height: 1.2em;
  display: flex;
  justify-content: space-between; 
  flex-wrap: wrap; 
}

textarea {
  background-color: #fff5f5;
  width : 100%; 
  flex-grow: 1; 
  min-height: 150px; 
  border-radius: 15px;
  padding: 10px 15px;
  box-shadow: 0 2px 4px rgba(255, 157, 157, 0.3);
  resize: none;
  border: none;
  font-family : inherit;
  box-sizing: border-box; 
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