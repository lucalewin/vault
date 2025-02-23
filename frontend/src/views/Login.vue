<template>
  <form v-on:submit.prevent="handleSubmit" class="form">
    <div>
      <label for="email">Email</label>
      <input type="email" id="email" v-model="email" class="input" required>
    </div>
    <div>
      <label for="password">Password</label>
      <input type="password" id="password" v-model="password" class="input" required>
    </div>
    <button type="submit">Log in</button>
    <!-- <div v-if="loading" class="loader"></div> -->
    <LinearSpinner :loading="loading" />
    <p v-if="error" style="color: red;">{{ error }}</p>
  </form>
</template>

<script setup>
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import LinearSpinner from '../components/LinearSpinner.vue';

const email = ref('');
const password = ref('');
const loading = ref(false);
const error = ref('');
const router = useRouter();

const handleSubmit = async () => {
  loading.value = true;
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
    if (data.status === 'error') {
      error.value = data.message;
    } else {
      const token = data.token;
      localStorage.setItem('api_token', token);
      router.push('/');
    }
  } catch (err) {
    console.error('Login failed:', err);
    error.value = 'Login failed. Please check your credentials and try again.';
  } finally {
    loading.value = false;
  }
};
</script>

<style scoped>
form {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  margin: 8px;
  padding: 1rem;
  border: 1px solid #ccc;
  border-radius: 4px;
}

form div {
  display: flex;
  flex-direction: column;
}

.loader {
  width: 100%;
  height: 4.8px;
  display: inline-block;
  position: relative;
  background: rgba(255, 255, 255, 0.15);
  overflow: hidden;
}
.loader::after {
  content: '';
  /* width: 192px; */
  width: 392px;
  height: 4.8px;
  background: lightskyblue;
  position: absolute;
  top: 0;
  left: 0;
  box-sizing: border-box;
  animation: animloader 1s linear infinite;
}

@keyframes animloader {
  0% {
    left: 0;
    transform: translateX(-100%);
  }
  100% {
    left: 100%;
    transform: translateX(0%);
  }
}

@media screen and (max-width: 440px) {
  form {
    border: none;
  }
}
</style>