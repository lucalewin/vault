<script setup lang="ts">
import { onMounted, ref, computed } from 'vue';
import { useRouter } from 'vue-router';

import Container from '@/components/Container.vue';

const router = useRouter();
const isLoggedIn = ref(false);
const credentials = ref([]);
interface Credential {
  id: string;
  service: string;
  [key: string]: any;
}

interface GroupedCredentials {
  [key: string]: Credential[];
}

const isTokenExpired = (token: string): boolean => Date.now() >= (JSON.parse(atob(token.split('.')[1]))).exp * 1000;

const fetchCredentials = async (): Promise<void> => {
  const token = localStorage.getItem('api_token');
  const response = await fetch('/api/v1/credentials', {
    headers: {
      'Authorization': `Bearer ${token}`
    }
  });
  const data = await response.json();
  credentials.value = data.credentials;
};

const groupedCredentials = computed<GroupedCredentials>(() => {
  return credentials.value.reduce((acc: GroupedCredentials, credential: Credential) => {
    if (!acc[credential.service]) {
      acc[credential.service] = [];
    }
    acc[credential.service].push(credential);
    return acc;
  }, {});
});

const sortedServices = computed<GroupedCredentials>(() => {
  const services = Object.keys(groupedCredentials.value);
  services.sort();
  return services.reduce((acc: GroupedCredentials, service: string) => {
    acc[service] = groupedCredentials.value[service];
    return acc;
  }, {});
});

const navigateToChallenge = (credentials: Credential[]): void => {
  const ids = credentials.map(credential => credential.id).join(',');
  router.push({ path: '/passwords/view', query: { ids } });
};

const export_passwords = (password: string): void => {
  // console.log(`Exporting passwords with password: ${password}`);
  // TODO
};
onMounted(() => {
  const token = localStorage.getItem('api_token');
  // const isTokenExpired = token => Date.now() >= (JSON.parse(atob(token.split('.')[1]))).exp * 1000;
  if (token && !isTokenExpired(token)) {
    isLoggedIn.value = true;
    fetchCredentials();
  }
});
</script>

<template>
  <h1 class="m-2 font-bold text-3xl">Passwords</h1>
  <Container v-if="!isLoggedIn">
    <div>
      <p>Please log in first!</p>
      <RouterLink to="/login">Log in</RouterLink>
    </div>
  </Container>
  <Container v-else>
    <div class="flex gap-2">
      <button class="px-4 py-2 font-semibold text-white bg-green-500/60 rounded hover:bg-green-700 focus:bg-green-700" v-on:click="router.push('/passwords/new')">New Credential</button>
      <button class="px-4 py-2 font-semibold text-white bg-green-500/60 rounded hover:bg-green-700 focus:bg-green-700" v-on:click="export_passwords('1234')">Export</button>
    </div>
    <div class="my-2 bg-nautral-800 rounded-xl overflow-hidden">
      <p class="px-4 py-2 border-b border-neutral-400 font-bold">
        Service
      </p>
      <div class="divide-y divide-neutral-700">
        <div
          v-for="(credentials, service) in sortedServices"
          :key="service"
          class="py-3 px-4 hover:bg-neutral-700 flex items-center"
          @click="navigateToChallenge(credentials)"
        >
          <i class="pi pi-globe mr-3"></i>
          <span class="flex-grow overflow-hidden mr-2">{{ service }}</span>
          <i class="pi pi-chevron-right"></i>
        </div>
      </div>
    </div>
    
  </Container>
</template>

<style scoped>
.secondary-button {
  background-color: #e2e8f0;
  color: #1a202c;
  padding: 0.5rem 1rem;
  border-radius: 0.375rem;
  display: inline-flex;
  align-items: center;
  cursor: pointer;
  margin-top: 1rem;
}
.secondary-button:hover {
  background-color: #cbd5e0;
}
a {
  color: black;
  text-decoration: none;
}

.services {
  margin: 8px;
  border: 1px solid #444; /* Less bright border */
  border-radius: 8px;
  overflow: hidden;
  background-color: #1e1e1e; /* Match background color with body */
}

.service {
  padding: 1em 16px;
  display: flex;
  align-items: center;
  gap: 1em;
  border-bottom: 1px solid #444; /* Less bright border */

  &:hover {
    background-color: #2a2a2a; /* Slightly lighter for hover effect */
    cursor: pointer;
  }
  &:last-child {
    border-bottom: none;
  }
  span {
    width: 100%;
  }
}

.new-credential {
  margin: 8px;
  padding: 1em 16px;
  border: 1px solid #444; /* Less bright border */
  border-radius: 8px;
  display: flex;
  gap: 1em;
  align-items: center;
  overflow: hidden;
  background-color: #1e1e1e; /* Match background color with body */
  
  &:hover {
    background-color: #2a2a2a; /* Slightly lighter for hover effect */
    cursor: pointer;
  }
}

</style>