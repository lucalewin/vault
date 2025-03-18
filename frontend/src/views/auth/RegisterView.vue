<script setup lang="ts">
import { ref } from 'vue';

const username = ref('');
const email = ref('');
const password = ref('');

const error = ref('');
const recovery_phrase = ref('');

const handleSubmit = async () => {
  try {
    const response = await fetch('/api/v1/auth/register', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        username: username.value,
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
      recovery_phrase.value = data.recovery_phrase;
    }
  } catch (err) {
    console.error('Login failed:', err);
    error.value = 'Login failed. Please check your credentials and try again.';
  }
};
</script>

<template>
  <div class="mx-auto mt-16 w-full max-w-sm">
    <form v-on:submit.prevent="handleSubmit" class="border border-neutral-600 bg-neutral-800 rounded-2xl p-6 space-y-4">
      <h2 class="text-2xl font-semibold">Register</h2>
      <p class="text-neutral-400">Create a new account</p>
      <div class="pt-4">
        <label for="username" class="block text-sm font-medium text-gray-300">Username</label>
        <input type="text" id="username" v-model="username" name="username" class="w-full px-3 py-2 mt-1 border border-neutral-600 rounded" />
      </div>
      <div>
        <label for="username" class="block text-sm font-medium text-gray-300">Email</label>
        <input type="text" id="username" v-model="email" name="email" class="w-full px-3 py-2 mt-1 border border-neutral-600 rounded" />
      </div>
      <div>
        <label for="password" class="block text-sm font-medium text-gray-300">Password</label>
        <input type="password" id="password" v-model="password" name="password" class="w-full px-3 py-2 mt-1 border border-neutral-600 rounded" />
      </div>
      <div>
        <button type="submit" class="w-full px-4 py-2 font-bold text-white bg-green-700/90 rounded hover:bg-green-700 focus:bg-green-700">Register</button>
      </div>
      <div class="text-xs text-neutral-400">
        Already have an account? <RouterLink to="/login" class="text-green-500">Sign in</RouterLink>
      </div>
      <div v-if="error">
        <p class="text-red-500">{{ error }}</p>
      </div>
      <div v-if="recovery_phrase">
        <textarea name="" id="" class="w-full border border-neutral-600">{{ recovery_phrase }}</textarea>
      </div>
    </form>
  </div>
</template>