<template>
  <div>
    <nav class="navbar">
      <img src="../../assets/unilim.png" alt="Logo" class="logo" />
      <ul>
        <li><router-link to="/prof">Acceuil</router-link></li>
        <li><router-link to="/studentlist">liste des élèves</router-link></li>
        <li><router-link to="/callsheet">Listes d'appel</router-link></li>
        <li><router-link to="/staffsheet">Liste personnel</router-link></li>
        <li><router-link to="/">Se déconnecter</router-link></li>
      </ul>
    </nav>

    <main class="callsheet-container">
      <h1>
        Liste d'appel du cours <span>&lt;RX.XX&gt;</span> de <span>&lt;XXhXX - XXhXX&gt;</span>
      </h1>

      <div class="student-list">
        <div class="student-list-header">
          <span>Nom / Prénom</span>
          <span>Promotion</span>
          <span>Groupe</span>
          <span>Aménagement</span>
          <span>Absent</span>
          <span>Retard (min)</span>
          <span>Commentaires</span>
        </div>

        <div v-for="student in students" :key="student.id" class="student-item">
          <span class="student-name">{{ student.name }}</span>
          <span>{{ student.promo }}</span>
          <span>{{ student.group }}</span>
          <span>{{ student.aménagement }}</span>
          
          <div class="input-cell">
            <input type="checkbox" v-model="student.isAbsent" class="custom-checkbox" />
          </div>
          
          <div class="input-cell">
            <input 
              type="number" 
              v-model.number="student.delay" 
              class="delay-input" 
              min="0" 
            />
          </div>
          
          <div class="input-cell">
            <textarea 
              v-model="student.comment" 
              class="comment-textarea" 
              rows="1"
            ></textarea>
          </div>
        </div>
      </div>
    </main>
  </div>
</template>

<script setup>
import { ref } from "vue";

const students = ref([
  { id: 1, name: 'MARTIN Lucas', promo: 'A2', group: 'G5-B', aménagement: 'NON', isAbsent: false, delay: null, comment: '' },
  { id: 2, name: 'DURAND Marie', promo: 'A2', group: 'G5-B', aménagement: 'NON', isAbsent: false, delay: null, comment: '' },
  { id: 3, name: 'PETIT Jean', promo: 'A2', group: 'G5-B', aménagement: 'Tiers-temps', isAbsent: false, delay: null, comment: '' },
  { id: 4, name: 'DUBOIS Alice', promo: 'A2', group: 'G5-B', aménagement: 'NON', isAbsent: false, delay: null, comment: '' },
  { id: 5, name: 'LEROY Tom', promo: 'A2', group: 'G5-B', aménagement: 'NON', isAbsent: false, delay: null, comment: '' },
  { id: 6, name: 'MOREAU Léa', promo: 'A2', group: 'G5-B', aménagement: 'NON', isAbsent: false, delay: null, comment: '' },
  { id: 7, name: 'GIRARD Hugo', promo: 'A2', group: 'G5-B', aménagement: 'NON', isAbsent: false, delay: null, comment: '' },
  { id: 8, name: 'BONNET Emma', promo: 'A2', group: 'G5-B', aménagement: 'NON', isAbsent: false, delay: null, comment: '' },
]);
</script>

<style scoped>
body {
  font-family: 'Plus Jakarta Sans', sans-serif;
  background-color: #f9f9f9;
  margin: 0;
  color: #791919; 
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

.callsheet-container {
  width: 90%;
  max-width: 1400px; 
  margin: 40px auto;
  color: #791919;
  font-family: 'Plus Jakarta Sans', sans-serif;
}

h1 {
  text-align: center;
  font-weight: 700;
  font-size: 40px;
  margin-bottom: 50px;
}

.student-list {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.student-list-header,
.student-item {
  display: grid;
  grid-template-columns: 2.5fr 1fr 1fr 1.5fr 0.6fr 0.8fr 3fr;
  gap: 20px;
  align-items: center;
}

.student-list-header {
  padding: 10px 20px;
  font-weight: 700;
  font-size: 14px;
  color: #555;
  border-bottom: 2px solid #ffdfdf;
}

.student-item {
  background-color: #fff5f5;
  border-radius: 15px;
  padding: 12px 20px;
  box-shadow: 0 2px 4px rgba(255, 157, 157, 0.3);
  font-weight: 500;
  font-size: 15px;
}

.student-item .student-name {
  font-weight: 700;
}

.input-cell {
  display: flex;
  justify-content: center;
  align-items: center;
}

.custom-checkbox {
  appearance: none;
  background-color: #ffffff;
  border: 1px solid #ffdfdf;
  border-radius: 8px;
  width: 25px;
  height: 25px;
  cursor: pointer;
  transition: background-color 0.2s;
}
.custom-checkbox:checked {
  background-color: #791919;
  border-color: #791919;
}

/* Input "Retard" */
.delay-input {
  width: 100%;
  padding: 8px;
  border: 1px solid #ffdfdf;
  border-radius: 8px;
  background-color: #ffffff;
  font-family: inherit;
  font-size: 14px;
  text-align: center;
  box-sizing: border-box;
}
.delay-input::-webkit-outer-spin-button,
.delay-input::-webkit-inner-spin-button {
  -webkit-appearance: none;
  margin: 0;
}

.comment-textarea {
  width: 100%;
  padding: 8px;
  border: 1px solid #ddd; 
  border-radius: 10px;
  background-color: #f0f0f0;
  font-family: inherit;
  font-size: 14px;
  resize: none;
  box-sizing: border-box;
  min-height: 38px;
}

.delay-input:focus,
.comment-textarea:focus {
  outline: none;
  border-color: #791919;
  box-shadow: 0 0 0 2px rgba(121, 25, 25, 0.2);
}
</style>