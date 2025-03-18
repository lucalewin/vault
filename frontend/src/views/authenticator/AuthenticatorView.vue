<template>
  <h1 class="p-1 font-semibold text-3xl">Authenticator Codes</h1>
  <div class="p-2 rounded-xl bg-neutral-800 max-w-lg mx-auto">
    <div v-if="isLoggedIn()" class="space-y-2">
      <button class="px-4 py-2 font-semibold text-white bg-green-500/60 rounded hover:bg-green-700 focus:bg-green-700" @click="showNewCodeModal = true">New Code</button>
      <ul class="space-y-1">
        <li v-for="entry in codes" :key="entry.id" class="px-2 py-1.5 bg-neutral-700/40 rounded-lg flex justify hover:bg-neutral-700 cursor-pointer" @click="copyToClipboard(entry.code)">
          <div>
            <strong>{{ entry.service }}</strong>
            <span class="ml-2 text-xs text-neutral-400" v-if="entry.username != ''">({{ entry.username }})</span>
            <p class="py-1 text-2xl text-green-600 font-semibold">
              {{ entry.code.slice(0, 3) + " " + entry.code.slice(3) }}
            </p>
          </div>
        </li>
      </ul>
    </div>
    <div v-else>
      To view codes: 
      <RouterLink to="/login" class="text-green-500">Login</RouterLink>
    </div>
  </div>

  <NewCodeModal :show="showNewCodeModal" @close="showNewCodeModal = false" />
</template>

<script setup lang="ts">
import { isLoggedIn } from '@/lib/api/auth';
import { copyToClipboard } from '@/lib/util';
import { ref, onMounted, onUnmounted } from 'vue';
import NewCodeModal from './NewCodeModal.vue';

const showNewCodeModal = ref(false);

interface AuthenticatorCode {
  id: number;
  service: string;
  username: string;
  code: string;
}

const codes = ref<AuthenticatorCode[]>([]);
let timeoutID: number | undefined = undefined;
let refreshIn = 30 - Math.round(new Date().getTime() / 1000) % 30 + 1;

const fetchCodes = async () => {
  const apiToken = localStorage.getItem('api_token');
  try {
    const response = await fetch('/api/v1/authenticator', {
      headers: {
        'Authorization': `Bearer ${apiToken}`,
      },
    });
    
    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`);
    }
    
    codes.value = await response.json();

    console.log("next refresh in " + refreshIn)
    timeoutID = setTimeout(fetchCodes, refreshIn * 1000); // Schedule the next fetch dynamically
    refreshIn = 30; // Reset refreshIn to 30 seconds
  } catch (error) {
    console.error('Failed to fetch authentication codes:', error);
    codes.value = [];
  }
};

if (isLoggedIn()) {
  onMounted(fetchCodes);
}

onUnmounted(async () => {
  if (timeoutID != undefined) {
    clearTimeout(timeoutID);
  }
})
</script>