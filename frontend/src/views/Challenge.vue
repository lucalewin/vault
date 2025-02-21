<template>
  <div>
    <h1>Challenge</h1>
    <form @submit.prevent="handleSubmit" class="form">
      <label for="master_password">Master Password</label>
      <input type="password" id="master_password" v-model="masterPassword" required>
      <button type="submit">Submit</button>
    </form>
    <div v-if="credentials.length">
      <h2>Credentials</h2>
      <ul>

        <li v-for="credential in credentials" :key="credential.id" class="credential">
          <strong for="username">Username:</strong> <p id="username">{{ credential.credential.username }}</p>
          <strong for="password">Password:</strong> <p id="password">{{ credential.credential.password }}</p>
        </li>
      </ul>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { useRoute } from 'vue-router';

const route = useRoute();
const masterPassword = ref('');
const credentials = ref([]);

const handleSubmit = async () => {
  const ids = route.query.ids.split(',');
  const token = localStorage.getItem('api_token');
  const fetchedCredentials = await Promise.all(ids.map(async (id) => {
    const response = await fetch(`/api/v1/credentials/${id}`, {
      method: 'POST',
      headers: {
        'Authorization': `Bearer ${token}`,
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ master_password: masterPassword.value }),
    });
    return response.json();
  }));
  credentials.value = fetchedCredentials;
  console.log(credentials.value);
};
</script>

<style scoped>
.form {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  max-width: 300px;
  margin: 0 auto;
  padding: 1rem;
  border: 1px solid #ccc;
  border-radius: 4px;
}
.credential {
  border: 1px solid #ccc;
  border-radius: 4px;
  padding: 1rem;
  margin-top: 1rem;
}
</style>
