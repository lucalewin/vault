<template>
  <h1 class="m-2 font-bold text-3xl">View Password</h1>
  <div class="mx-auto mt-16 w-full max-w-sm">
    <form v-if="!credentials.length" @submit.prevent="handleSubmit" class="border border-neutral-600 bg-neutral-800 rounded-2xl p-6 space-y-4">
      <h1>Challenge for {{ service }}</h1>
      <div>
        <label for="master_password">Master Password</label>
        <input type="password" id="master_password" v-model="masterPassword" class="w-full px-3 py-2 mt-1 border border-neutral-600 rounded" required>
      </div>
      <div>
        <button type="submit" class="w-full px-4 py-2 font-semibold text-white bg-green-500/60 rounded hover:bg-green-700 focus:bg-green-700">View</button>
      </div>
    </form>
    <div v-if="credentials.length" class="border border-neutral-600 bg-neutral-800 rounded-2xl p-6 space-y-4">
      <h1>Credentials for {{ service }}</h1>
      <ul class="credentials-list">
        <li v-for="credential in credentials" :key="credential.id" class="credential">
          <div>
            <label for="username">Username:</label>
            <div class="input-container">
              <input type="text" id="username" :value="credential.credential.username" class="input" readonly>
              <span class="input-icons">
                <span @click="copyToClipboard(credential.credential.username)"><i class="pi pi-copy"></i></span>
              </span>
            </div>
          </div>
          <div>
            <label for="password">Password:</label>
            <div class="input-container bg-neutral-800">
              <input :type="credential.showPassword ? 'text' : 'password'" id="password" :value="credential.credential.password" class="input" readonly>
              <span class="flex absolute items-center right-0 h-full text-neutral-400">
                <i class="pi pi-eye-slash hover:bg-neutral-600 rounded-full p-1.5" @click="toggleShowPassword(credential)"></i>
                <i class="pi pi-copy hover:bg-neutral-600 rounded-full p-1.5" @click="copyToClipboard(credential.credential.password)"></i>
              </span>
            </div>
          </div>
        </li>
      </ul>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useRoute } from 'vue-router';
// import LinearSpinner from '../components/LinearSpinner.vue';
import { copyToClipboard } from '../../lib/util';

import Container from '@/components/Container.vue';

const route = useRoute();
const masterPassword = ref('');
interface Credential {
  id: string;
  credential: {
    username: string;
    password: string;
    service: string;
  };
  showPassword: boolean;
}

const credentials = ref<Credential[]>([]);
const loading = ref(false);
const service = ref('');

const handleSubmit = async () => {
  loading.value = true;
  const ids = (route.query.ids as string)?.split(',');
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
    const credential = await response.json();
    credential.showPassword = false;
    return credential;
  }));
  credentials.value = fetchedCredentials;
  if (credentials.value.length > 0) {
    service.value = credentials.value[0].credential.service;
  }
  console.log(credentials.value);
  loading.value = false;
};

const toggleShowPassword = (credential: Credential) => {
  credential.showPassword = !credential.showPassword;
};
</script>

<style scoped>
.form {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  margin: 8px;
  padding: 1rem;
  border: 1px solid #444; /* Less bright border */
  border-radius: 4px;
}

form div {
  display: flex;
  flex-direction: column;
}

.credentials-list {
  list-style-type: none;
  padding: 0;
  margin-bottom: 1rem; /* Add margin to the bottom */
}

.credential {
  border: 1px solid #444; /* Less bright border */
  border-radius: 8px;
  padding: 1rem;
  margin: 0 8px;
  margin-top: 1rem;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.input-container {
  display: flex;
  align-items: center;
  position: relative;
}

.input-container input {
  flex: 1;
  padding-right: 85px;
  outline: none;
}

.input-icons {
  display: flex;
  position: absolute;
  align-items: center;
  right: 0;
  height: 100%;
  z-index: 1000;
  margin-right: 12px;
}

.input-icons span {
  background: none;
  border: none;
  cursor: pointer;
}

@media screen and (max-width: 440px) {
  form {
    border: none;
  }
}
</style>
