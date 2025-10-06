<template>
    <div class="navbar">
        <img src="../src\assets\unilim.png" alt="Logo" class="logo" />
        <ul>
            <router-link to="/first" id="nav-links">Menu</router-link>
            <router-link to="/cours" id="nav-links">Cours</router-link>
            <router-link to="/ressources" id="nav-links">Ressources</router-link>
        </ul>
    </div>
    <div class="content">
        <div class="selection_group">
            <div class="content_selection_group">
                <p>choisir les groupes des A1</p>
                <ul >
                    <router-link class="selection">G1A</router-link>
                    <router-link class="selection">G1B</router-link>
                    <router-link class="selection">G2A</router-link>
                    <router-link class="selection">G2B</router-link>
                    <router-link class="selection">G3A</router-link>
                    <router-link class="selection">G3B</router-link>
                </ul>

            </div>
        </div>
        <div class="selection_group">
            <div class="content_selection_group">
                <p>choisir les groupes des A2</p>
                <ul>
                    <router-link class="selection">G4A</router-link>
                    <router-link class="selection">G4B</router-link>
                    <router-link class="selection">G5A</router-link>
                    <router-link class="selection">G5B</router-link>
                </ul>
            </div>
        </div>
        <div class="selection_group">
            <div class="content_selection_group">
                <p>choisir les groupes des A3</p>
                <router-link class="selection">G6A</router-link>
                <router-link class="selection">G6B</router-link>
                <router-link class="selection">G7A</router-link>
                <router-link class="selection">G7B</router-link>
            </div>
        </div>
    </div>

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
    background-color: #791919;
    border: 0.5vmin solid #05060f;
    box-shadow: 0.4rem 0.4rem #05060f;
    overflow: hidden;
    color: black;
}

#nav-links {
    margin: 20px;
    color: white;
    text-decoration: none;
    font-weight: bold;
}


.content {
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 100px;
}

ul {
    display: block;
    list-style-type: disc;
    margin-top: 1em;
    margin-bottom: 1 em;
    margin-left: 0;
    margin-right: 0;
    padding-left: 40px;
}

.selection {
    display: list-item;
    margin: 20px;
    color: white;
    text-decoration: none;
}

.selection_group {
    width: auto;
    background: #791919;
    color: white;
    padding: 1rem;
    border-radius: 1rem;
    border: 0.5vmin solid #05060f;
    box-shadow: 0.4rem 0.4rem #05060f;
    overflow: hidden;
    color: black;
}

.content_selection_group {
    color: white;
    font-size: 1.3rem;
    line-height: 1.25;
    font-weight: 700;
}
</style>
