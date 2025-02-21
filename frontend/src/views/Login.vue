<template>
  <form v-on:submit.prevent="handleSubmit" class="form">
    <div>
      <label for="email">Email</label>
      <input type="email" id="email" v-model="email" required>
    </div>
    <div>
      <label for="password">Password</label>
      <input type="password" id="password" v-model="password" required>
    </div>
    <button type="submit">Log in</button>
  </form>
</template>

<script setup>
import { ref } from 'vue';

const email = ref('');
const password = ref('');

const handleSubmit = async () => {
  try {
    const response = await fetch('/api/v1/auth/login', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        email: email.value,
        password: password.value,
      }),
    });

    if (!response.ok) {
      throw new Error('Network response was not ok');
    }

    const data = await response.json();
    const token = data.token;
    localStorage.setItem('api_token', token);
    alert('Login successful!');
  } catch (error) {
    console.error('Login failed:', error);
    alert('Login failed. Please check your credentials and try again.');
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

/* button {
  outline: none;
  border: none;
  background-color: darkorange;
  border-radius: 8px;
  line-height: 2.5em;
  font-size: 16px;
} */

@media screen and (max-width: 440px) {
  form {
    border: none;
  }
}
</style>