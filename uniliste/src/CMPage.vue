<template>
    <head class="navbar">
        <img src="../src\assets\unilim.png" alt="Logo" class="logo" />
        <ul>
            <router-link to="/first" id="nav-links">Menu</router-link>
            <router-link to="/groups" id="nav-links">Groupes</router-link>
            <router-link to="/ressources" id="nav-links">Ressources</router-link>
            <router-link to="/cours" id="nav-links">Cours</router-link>
            <router-link to="/about" id="nav-links">A Propos</router-link>
        </ul>
    </head>
    <body>
        <div>
            <p>choisir promo</p>
            <ul>
                <router-link class="selection">A1</router-link>
                <router-link class="selection">A2</router-link>
                <router-link class="selection">A3</router-link>
            </ul>
        </div>
    </body>
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
    align-items: center;
    background-color: #791919;
    border: 0.5vmin solid #05060f;
    box-shadow: 0.4rem 0.4rem #05060f;
    overflow: hidden;
    color: black;
}

#nav-links {
    color: white;
    padding: 20px;
    text-decoration: none;
    font-weight: bold;
}

.content {
    min-height: 400px;
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 100px;
}

#cours-links {
    margin: 20px;
    color: white;
    text-decoration: none;
}

.selection_cours {
    width: 190px;
    background: #791919;
    color: white;
    padding: 1rem;
    border-radius: 1rem;
    border: 0.5vmin solid #05060f;
    box-shadow: 0.4rem 0.4rem #05060f;
    overflow: hidden;
    color: black;
}

.content_selection_cours {
    color: white;
    text-align: center;
    font-size: 1.3rem;
    line-height: 1.25;
    font-weight: 700;
}
</style>
