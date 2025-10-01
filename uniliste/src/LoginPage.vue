<template>
  <main class="login-page">
    <div class="login-container">
      <input
        v-model="username" type="text" placeholder="Identifiant" class="input-field"/>

      <input
        v-model="password"
        type="password"
        placeholder="Password"
        class="input-field"
      />

      <div class="forgot-password">
        <a href="#" class="forgot-password-link">Mot de passe oublié ?</a>
      </div>

      <button @click="verification" class="login-button">
        Se connecter
      </button>
      <p>{{ msg }}</p>
    </div>
  </main>
</template>

<script setup>
import { ref } from "vue";
import { useRouter } from "vue-router";

const username = ref('')
const password = ref('')
const msg = ref('')
const router = useRouter()

const verification = () => {
  if (username.value == 'azerty' && password.value == '0000') {
    msg.value = "Connexion réussie"
    router.push("/first")

  } else {
    msg.value = "Veuillez remplir tous les champs"
  }
}

</script>
<style scoped>
.login-page {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 100vh;
  background-color: white;
}

.login-container {
  display: flex;
  flex-direction: column;
  gap: 16px;
  width: 320px;
}

.input-field {
  border: none;
  border-radius: 9999px; 
  padding: 12px 16px;
  background-color: #f4f6ff; 
  font-size: 14px;
  outline: none;
}

.input-field:focus {
  box-shadow: 0 0 0 2px #d0d4f7;
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
  border-radius: 9999px;
  padding: 12px;
  background-color: #791919;
  color: white;
  font-weight: 600;
  cursor: pointer;
  transition: background-color 0.2s;
}

.login-button:hover {
  background-color: #520f0f;
}
</style>
