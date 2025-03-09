<template>
  <div v-if="!isLoggedIn">
    <button class="button" @click="router.push('/login')">Login</button>
    <button class="button" @click="router.push('/register')">Register</button>
  </div>
  <div class="container" v-else>
    <div class="services">
      <div
        v-for="(credentials, service) in sortedServices"
        :key="service"
        class="service"
        @click="navigateToChallenge(service, credentials)"
      >
        <i class="pi pi-globe"></i>
        <span>{{ service }}</span>
        <i class="pi pi-chevron-right"></i>
      </div>
    </div>
    <div class="new-credential" v-on:click="router.push('/new')">
      <i class="pi pi-plus"></i>
      <span>New credential</span>
    </div>
  </div>
</template>

<script setup>
import { onMounted, ref, computed } from 'vue';
import { useRouter } from 'vue-router';

const router = useRouter();
const isLoggedIn = ref(false);
const credentials = ref([]);

onMounted(() => {
  const token = localStorage.getItem('api_token');
  const isTokenExpired = token => Date.now() >= (JSON.parse(atob(token.split('.')[1]))).exp * 1000;
  if (token && !isTokenExpired(token)) {
    isLoggedIn.value = true;
    fetchCredentials();
  }
});

const fetchCredentials = async () => {
  const token = localStorage.getItem('api_token');
  const response = await fetch('/api/v1/credentials', {
    headers: {
      'Authorization': `Bearer ${token}`
    }
  });
  const data = await response.json();
  credentials.value = data.credentials;
};

const groupedCredentials = computed(() => {
  return credentials.value.reduce((acc, credential) => {
    if (!acc[credential.service]) {
      acc[credential.service] = [];
    }
    acc[credential.service].push(credential);
    return acc;
  }, {});
});

const sortedServices = computed(() => {
  const services = Object.keys(groupedCredentials.value);
  services.sort();
  return services.reduce((acc, service) => {
    acc[service] = groupedCredentials.value[service];
    return acc;
  }, {});
});

const navigateToChallenge = (service, credentials) => {
  const ids = credentials.map(credential => credential.id).join(',');
  router.push({ path: '/view', query: { ids } });
};
</script>

<style scoped>
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

.container {
  margin-bottom: 1600px;
}

.button {
  margin: 8px;
}
</style>