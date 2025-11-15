<template>
  <div>
    <nav class="navbar">
      <img src="../../assets/unilim.png" alt="Logo" class="logo" />
      <ul>
        <li><router-link to="/prof">Acceuil</router-link></li>
        <li><router-link to="/sudentlist">liste des élèves</router-link></li>
        <li><router-link to="/callsheet">Listes d'appel</router-link></li>
        <li><router-link to="/stafflist">Liste du personnel</router-link></li>
        <li><router-link to="/">Se déconnecter</router-link></li>
      </ul>
    </nav>
  </div>
</template>