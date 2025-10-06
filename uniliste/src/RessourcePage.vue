<template>
    <div class="navbar">
        <img src="../src\assets\unilim.png" alt="Logo" class="logo" />
        <ul>
            <router-link to="/first" id="nav-links">Menu</router-link>
            <router-link to="/cours" id="nav-links">Cours</router-link>
            <router-link to="/groups" id="nav-links">Groupes</router-link>

        </ul>
    </div>
    <h1>Bienvenue sur resources</h1>
</template>

<script setup>
import { ref } from "vue";
import { useRouter } from "vue-router";
const router = useRouter()

const home = () => {
    router.push("/first")
}
</script>
<style scoped>
.logo {
    width: 70px;
    border-radius: 10px;
    margin: 10px;
}



.navbar {
    display: flex;
    gap: 20px;
    align-items: center;
    background-color: #791919;
    border: 0.5vmin solid #05060f;
    box-shadow: 0.4rem 0.4rem #05060f;
    overflow: hidden;
    color: black;
}

#nav-links {
    color: white;
    margin: 20px;
    text-decoration: none;
}
</style>
