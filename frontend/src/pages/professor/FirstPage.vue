<template>
  <div>
    <nav class="navbar">
      <img src="../assets/unilim.png" alt="Logo" class="logo" />
      <ul>
        <li><router-link to="/prof">Menu</router-link></li>
        <li><router-link to="/list">liste des élèves</router-link></li>
        <li><router-link to="/ajouter-eleve">Listes d'appel</router-link></li>
        <li><router-link to="/supprimer-eleve">Liste personnel</router-link></li>
        <li><router-link to="/modifier-eleve">???</router-link></li>
        <li><router-link to="/">Se déconnecter</router-link></li>
      </ul>
    </nav>

    <main class="dashboard-container">
      <h1>Bonjour M. Hugel</h1>

      <div class="dashboard-grid">
        
        <section class="grid-column">
          <h2>Vos prochains cours</h2>
          <div class="course-card-container">
            <div v-for="course in nextCourses" :key="course.title" class="course-card">
              <span class="course-title">{{ course.title }}</span>
              <span class="course-details">{{ course.details }}</span>
              <span class="course-time">{{ course.time }}</span>
            </div>
          </div>
        </section>

        <section class="grid-column">
          <div class="module">
            <h2>Appel à faire</h2>
            <div class="list-container">
              <div v-for="call in callsToDo" :key="call.course" class="list-item">
                <span>{{ call.course }}</span>
                <span>{{ call.time }}</span>
                <span class="status-red">À faire</span>
              </div>
            </div>
          </div>
          <div class="module">
            <h2>Absences des élèves</h2>
            <div class="list-container">
              <div v-for="absence in studentAbsences" :key="absence.name" class="list-item absence">
                <span class="student-name">{{ absence.name }}</span>
                <span>{{ absence.group }}</span>
                <span :class="absence.status === 'Justifiée' ? 'status-green' : 'status-red'">
                  {{ absence.status }}
                </span>
              </div>
            </div>
          </div>
        </section>

        <section class="grid-column">
          <div class="module">
            <h2>Mesures disciplinaires</h2>
            <p class="subtitle">Aucune mesure disciplinaire n'a été prise.</p>
          </div>
          <div class="module">
            <h2>Absence personnel</h2>
            <div class="list-container">
              <div v-for="staff in staffAbsences" :key="staff.name" class="list-item staff">
                <span class="staff-name">{{ staff.name }}</span>
                <span class="staff-role">{{ staff.role }}</span>
              </div>
            </div>
          </div>
        </section>

      </div>
    </main>
  </div>
</template>

<script setup>
import { ref } from "vue";
const nextCourses = ref([
  { 
    title: 'CM - Amphitheatre C', 
    details: 'R1.01 - Initiation au développement', 
    time: '10h30 - 12h00' 
  },
  { 
    title: 'TD - 209', 
    details: 'R3.05 - Systèmes d\'exploitations', 
    time: '13h30 - 15h30' 
  },
]);

const callsToDo = ref([
  { course: 'Cours R3.01', time: '8h00 - 10h00' },
  { course: 'Cours R2.06', time: '10h00 - 12h00' },
]);

const studentAbsences = ref([
  { name: 'DEGABRIEL Enzo', group: 'A2 G5-B', status: 'Justifiée' },
  { name: 'VALET Martin', group: 'A2 G4-B', status: 'Injustifiée' },
  { name: 'PASTEL Gaël', group: 'A2 G5-B', status: 'Justifiée' },
]);

const staffAbsences = ref([
  { name: 'HUGEL Thomas', role: 'Enseignant' },
  { name: 'BOULEISTEIX Claudie', role: 'Secrétaire' },
]);
</script>

<style>
body {
  font-family: 'Plus Jakarta Sans', sans-serif;
  background-color: #fafafa;
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

.dashboard-container {
  width: 90%;
  max-width: 1400px;
  margin: 0 auto;
  padding: 40px 20px;
}

.dashboard-container h1 {
  text-align: center;
  font-size: 40px;
  font-weight: 700;
  margin-top: 0;
  margin-bottom: 40px;
}

.dashboard-grid {
  display: grid;
  grid-template-columns: 1.2fr 1fr 1fr; 
  gap: 30px;
}

.grid-column {
  display: flex;
  flex-direction: column;
  gap: 30px; 
}

.grid-column h2 {
  font-size: 24px;
  font-weight: 600;
  margin-top: 0;
  margin-bottom: 10px; 
}

.list-container, .course-card-container {
  display: flex;
  flex-direction: column;
  gap: 15px;
}

.course-card {
  background-color: #fff5f5;
  border-radius: 15px;
  padding: 20px;
  box-shadow: 0 2px 4px rgba(255, 157, 157, 0.3);
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.course-title {
  font-weight: 700;
  font-size: 16px;
}
.course-details, .course-time {
  font-size: 15px;
  color: #3d0000;
}
.course-time {
  align-self: flex-end;
  font-weight: 600;
}

.list-item {
  background-color: #fff5f5;
  border-radius: 15px;
  padding: 14px 18px;
  box-shadow: 0 2px 4px #3d0000;
  display: grid;
  align-items: center;
  font-size: 14px;
  font-weight: 500;
  
  grid-template-columns: 1fr 1fr auto;
  gap: 15px;
}

.list-item.absence {
  grid-template-columns: 1.5fr 1fr auto;
}
.student-name {
  font-weight: 700;
}

.list-item.staff {
  grid-template-columns: 1.5fr 1fr;
}
.staff-name {
  font-weight: 700;
}
.staff-role {
  font-size: 14px;
  color: #791919;
}

.status-red {
  color: #D93030;
  font-weight: 700;
  justify-self: end;
}
.status-green {
  color: #28a745;
  font-weight: 700;
  justify-self: end;
}

.subtitle {
  font-size: 14px;
  color : #791919;
  font-style: italic;
  margin: 0;
}
</style>