<script setup lang="ts">
import { ref } from 'vue';
import { useRouter } from 'vue-router';

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

<template>
  <div class="flex items-center justify-center">
    <form v-on:submit.prevent="handleSubmit" class="w-full max-w-sm p-8 mt-16 space-y-4 sm:border border-gray-700 rounded shadow-md">
      <div>
        <label for="username" class="block text-sm font-medium text-gray-300">Username</label>
        <input type="text" id="username" v-model="email" name="username" class="w-full px-3 py-2 mt-1 border border-gray-700 rounded" />
      </div>
      <div>
        <label for="password" class="block text-sm font-medium text-gray-300">Password</label>
        <input type="password" id="password" v-model="password" name="password" class="w-full px-3 py-2 mt-1 border border-gray-700 rounded" />
      </div>
      <div>
        <button type="submit" class="w-full px-4 py-2 font-bold text-white bg-blue-500 rounded hover:bg-blue-700">Login</button>
      </div>
    </form>
  </div>
</template>