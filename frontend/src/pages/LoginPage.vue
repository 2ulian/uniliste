<template>
  <main class="login-page">
    <div class="welcome-message">
      <img src="../assets/unilim.png" alt="Logo" class="logo" />
      <h1>Bienvenue sur Uniliste</h1>
    </div>

    <div class="login-container">
      <input 
        v-model="username" 
        type="text" 
        placeholder="Identifiant" 
        class="input-field"
      />
      <input 
        v-model="password" 
        type="password" 
        placeholder="Mot de passe" 
        class="input-field"
      />

      <div class="forgot-password">
        <a href="#" class="forgot-password-link">Mot de passe oublié ?</a>
      </div>

      <button @click="verification" class="login-button">
        Se connecter
      </button>
      
      <p v-if="msg" class="error-message">{{ msg }}</p>
    </div>
  </main>
</template>

<script setup>
import { ref } from "vue";
import { useRouter } from "vue-router";

const username = ref('');
const password = ref('');
const msg = ref('');
const router = useRouter();

const verification = () => {
  if (!username.value || !password.value) {
    msg.value = "Veuillez remplir tous les champs";
  } 
  else if (username.value === 'admin' && password.value === 'admin') {
    router.push("/first");
  } 
  else if (username.value === 'prof' && password.value === 'prof') {
    router.push("/prof");
  } 
  else {
    msg.value = "Identifiant ou mot de passe incorrect";
  }
}
</script>

<style scoped>

.login-page {
  display: flex;
  flex-direction: column; 
  align-items: center;    
  justify-content: flex-start; 
  padding-top: 10vh;     
  min-height: 100vh;
  background-color: white;
  gap: 40px; 
  font-family: 'Plus Jakarta Sans', sans-serif; 
}

.welcome-message {
  display: flex;
  align-items: center; 
  justify-content: center;
}

.welcome-message .logo {
  width: 60px; 
  margin-right: 15px; 
}

.welcome-message h1 {
  font-family: 'Plus Jakarta Sans', sans-serif;
  color: #791919; 
  font-size: 28px; 
  font-weight: 700; 
  margin: 0;       
}

.login-container {
  display: flex;
  flex-direction: column;
  gap: 16px;
  width: 320px;
}

.input-field {
  border: none;
  border-radius: 100px; 
  padding: 12px 16px;
  background-color: #fff5f5; 
  font-size: 14px;
  outline: none; 
  font-family: inherit; 
}

.input-field:focus {
  box-shadow: 0 0 0 2px #ffdfdf; 
}

.forgot-password {
  display: flex;
  justify-content: flex-end;
}

.forgot-password-link {
  font-size: 13px;
  color: #822d2d; 
  text-decoration: none;
}

.forgot-password-link:hover {
  text-decoration: underline;
}

.login-button {
  border: none;
  border-radius: 100px;
  padding: 12px;
  background-color: #791919;
  color: white;
  font-weight: 600;
  cursor: pointer;
  transition: background-color 0.2s;
  font-family: inherit;
  font-size: 15px;
}

.login-button:hover {
  background-color: #520f0f;
}

.error-message {
  color: #791919;
  font-size: 14px;
  text-align: center;
  margin: 0;
}
</style>