<template>
  <form @submit.prevent="handleSubmit">
    <!-- service, username, password, master_password -->
    <h1>New credential</h1>
    <div>
      <label for="service">Service</label>
      <input type="text" id="service" v-model="service" required>
    </div>
    <div>
      <label for="username">Username</label>
      <input type="text" id="username" v-model="username" required>
    </div>
    <div>
      <label for="password">Password</label>
      <input type="password" id="password" v-model="password" required>
    </div>
    <div>
      <label for="master_password">Master Password</label>
      <input type="password" id="master_password" v-model="master_password" required>
    </div>
    <button type="submit">Save</button>
  </form>
</template>

<script setup>
import { ref } from 'vue';
import { useRouter } from 'vue-router';

const router = useRouter();
const service = ref('');
const username = ref('');
const password = ref('');
const master_password = ref('');

const handleSubmit = async () => {
  try {
    const token = localStorage.getItem('api_token');
    const response = await fetch('/api/v1/credentials', {
      method: 'POST',
      headers: {
        'Authorization': `Bearer ${token}`,
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        service: service.value,
        username: username.value,
        password: password.value,
        master_password: master_password.value,
      }),
    });

    if (!response.ok) {
      throw new Error('Network response was not ok');
    }

    alert('Credential saved successfully!');
    router.push('/');
  } catch (error) {
    console.error('Failed to save credential:', error);
    alert('Failed to save credential. Please try again.');
  }
};
</script>

<style scoped>
form {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  /* max-width: 300px; */
  margin: 8px;
  padding: 1rem;
  border: 1px solid #ccc;
  border-radius: 4px;
}

form div {
  display: flex;
  flex-direction: column;
}

@media screen and (max-width: 440px) {
  form {
    border: none;
  }
}
</style>