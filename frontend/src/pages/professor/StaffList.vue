<template>
  <div>
    <nav class="navbar">
      <img src="../../assets/unilim.png" alt="Logo" class="logo" />
      <ul>
        <li><router-link to="/prof">Acceuil</router-link></li>
        <li><router-link to="/list">liste des élèves</router-link></li>
        <li><router-link to="/ajouter-eleve">Listes d'appel</router-link></li>
        <li><router-link to="/supprimer-eleve">Liste personnel</router-link></li>
        <li><router-link to="/">Se déconnecter</router-link></li>
      </ul>
    </nav>
  </div>
</template>