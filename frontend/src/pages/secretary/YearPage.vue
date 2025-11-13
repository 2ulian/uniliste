<template>

    <head class="navbar">
        <img src="../../assets/unilim.png" alt="Logo" class="logo" />
        <ul>
            <li><router-link to="/first" id="nav-links">Menu</router-link></li>
            <li><router-link to="/student" id="nav-links">Importer étudiant</router-link></li>
            <li><router-link to="/group" id="nav-links">Importer groupes</router-link></li>
            <li><router-link to="/ressources" id="nav-links">Importer ressources</router-link></li>
            <li><router-link to="/professor" id="nav-links">Importer professeur</router-link></li>
            <li><router-link to="/ressources" id="nav-links">Saisir justificatif</router-link></li>
            <li><router-link to="/" id="nav-links">Quitter</router-link></li>
            
        </ul>
    </head>

    <body>
        <p>annee universitaire</p>
    </body>
</template>

<script setup>

</script>
<style scoped>
.logo {
  width: 60px;
  height: auto; 
  border-radius: 10px;
  margin: 10px ;
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
</style>
