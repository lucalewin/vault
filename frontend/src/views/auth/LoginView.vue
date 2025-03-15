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
  <div class="mx-auto mt-16 w-full max-w-sm">
    <form v-on:submit.prevent="handleSubmit" class="border border-neutral-600 bg-neutral-800 rounded-2xl p-6 space-y-4">
      <h2 class="text-2xl font-semibold">Sign In</h2>
      <p class="text-neutral-400">Enter your email and password to sign in</p>
      <div class="pt-4">
        <label for="username" class="block text-sm font-medium text-gray-300">Username</label>
        <input type="text" id="username" v-model="email" name="username" class="w-full px-3 py-2 mt-1 border border-neutral-600 rounded" />
      </div>
      <div>
        <label for="password" class="block text-sm font-medium text-gray-300">Password</label>
        <input type="password" id="password" v-model="password" name="password" class="w-full px-3 py-2 mt-1 border border-neutral-600 rounded" />
      </div>
      <div class="flex justify-between text-neutral-400 text-xs items-center">
        <div class="flex items-center">
          <div class="flex items-center cursor-pointer relative mr-1">
            <input type="checkbox" class="peer h-4.5 w-4.5 cursor-pointer appearance-none rounded shadow hover:shadow-md border border-neutral-600 checked:bg-green-600/80 checked:border-green-800" id="check" />
            <span class="absolute text-white opacity-0 peer-checked:opacity-100 top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 pointer-events-none">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-3.5 w-3.5" viewBox="0 0 20 20" fill="currentColor" stroke="currentColor" stroke-width="1">
                <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd"></path>
              </svg>
            </span>
          </div>
          <label>Keep me logged in</label>
        </div>
        <RouterLink class="text-green-500" to="/forgot-password">Forgot password?</RouterLink>
      </div>
      <div>
        <button type="submit" class="w-full px-4 py-2 font-bold text-white bg-green-700/90 rounded hover:bg-green-700 focus:bg-green-700">Login</button>
      </div>
      <div class="text-xs text-neutral-400">
        Don't have an account? <RouterLink to="/register" class="text-green-500">Sign up</RouterLink>
      </div>
    </form>
  </div>
</template>