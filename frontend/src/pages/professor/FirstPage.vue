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
  { name: 'RAY-CONSTANTY Julian', group: 'A2 G5-B', status: 'Justifiée' },
]);

const staffAbsences = ref([
  { name: 'HUGEL Thomas', role: 'Enseignant' },
  { name: 'BOULEISTEIX Claudie', role: 'Secrétaire' },
  { name: 'ESTRADE Clément', role: 'vacataire' },
]);
</script>

<style>
body {
  font-family: 'Plus Jakarta Sans', sans-serif;
  background-color: #fafafa; 
  margin: 0;
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

.grid-container {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(400px, 1fr));
  grid-template-rows: repeat(2, auto);
  gap: 30px;
  padding: 40px;
  color: #791919;
}

.grid-item {
  background-color: white;
  border-radius: 15px;
  padding: 20px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.05); 
  display: flex;
  flex-direction: column;
}

h1, h2 {
  margin-top: 0;
}

.info-box {
  width: 100%;
  flex-grow: 1;
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.info-item {
  background-color: #fff5f5;
  border-radius: 15px;
  padding: 10px 15px;
  box-shadow: 0 2px 4px rgba(255, 157, 157, 0.3);
  font-size : 14px;
  min-height: 1.2em;
  display: flex;
  justify-content: space-between; 
  flex-wrap: wrap; 
}

textarea {
  background-color: #fff5f5;
  width : 100%; 
  flex-grow: 1; 
  min-height: 150px; 
  border-radius: 15px;
  padding: 10px 15px;
  box-shadow: 0 2px 4px rgba(255, 157, 157, 0.3);
  resize: none;
  border: none;
  font-family : inherit;
  box-sizing: border-box; 
}

.course-title {
 font-weight: 700;
 font-size: 18px; /* MODIFIÉ (était 1.1rem) */
}
.course-details, .course-time {
 font-size: 16px; /* MODIFIÉ (était 0.95rem) */
 color: #8c5353;
}
.course-time {
 align-self: flex-end; 
 font-weight: 600;
}

.list-item {
 background-color: #fff5f5;
 border-radius: 15px;
 padding: 18px 22px;
 box-shadow: 0 2px 4px rgba(255, 157, 157, 0.3);
 display: grid;
 align-items: center;
 font-size: 15px; /* MODIFIÉ (était 0.9rem) */
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
 font-size: 14px; /* MODIFIÉ (était 0.85rem) */
 color: #8c5353;
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
 font-size: 15px; /* MODIFIÉ (était 0.9rem) */
 color: #791919; /* MODIFIÉ (était #eee) */
 font-style: italic;
 margin: 0;
}
</style>