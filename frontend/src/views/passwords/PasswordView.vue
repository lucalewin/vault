<script setup lang="ts">
import { onMounted, ref, computed } from 'vue';
import { useRouter } from 'vue-router';

import Container from '@/components/Container.vue';
import { export_passwords, import_passwords } from '@/lib/api/credentials';
import Modal from '../../components/Modal.vue';
import ImportModal from './ImportModal.vue';

const router = useRouter();
const isLoggedIn = ref(false);
const credentials = ref([]);
const showExportModal = ref(false);
const showImportModal = ref(false);
const masterPassword = ref('');
const importJSON = ref('');

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

const handleExport = (): void => {
  export_passwords(masterPassword.value);
  showExportModal.value = false;
};

const handleImport = (): void => {
  console.log("import!!!")
  console.log(importJSON.value)
  import_passwords(JSON.parse(importJSON.value), masterPassword.value);
  showImportModal.value = false;
}

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
      <button class="px-4 py-2 font-semibold text-white bg-green-500/60 rounded hover:bg-green-700 focus:bg-green-700" v-on:click="showExportModal = true">Export</button>
      <button class="px-4 py-2 font-semibold text-white bg-green-500/60 rounded hover:bg-green-700 focus:bg-green-700" v-on:click="showImportModal = true">Import</button>
    </div>
    <div class="my-2 bg-nautral-800 rounded-xl overflow-hidden">
      <p class="px-4 py-2 border-b border-neutral-400 font-bold">
        Service
      </p>
      <div class="divide-y divide-neutral-700">
        <div
          v-for="(credentials, service) in sortedServices"
          :key="service"
          class="py-3 px-4 hover:bg-neutral-700 flex items-center cursor-pointer"
          @click="navigateToChallenge(credentials)"
        >
          <i class="pi pi-globe mr-3"></i>
          <span class="flex-grow overflow-hidden mr-2">{{ service }}</span>
          <i class="pi pi-chevron-right"></i>
        </div>
      </div>
    </div>
    
  </Container>

  <!-- Export Modal -->
  <Modal :show="showExportModal" @close="showExportModal = false">
    <template #header>
      <h2 class="text-xl font-bold mb-4">Enter Master Password</h2>
    </template>

    <template #body>
      <input
        type="password"
        v-model="masterPassword"
        class="w-full px-4 py-2 border rounded mb-4"
        placeholder="Master Password"
      />
    </template>

    <template #footer>
      <div class="flex justify-end gap-2">
        <button class="px-4 py-2 bg-neutral-600 rounded hover:bg-neutral-700" @click="showExportModal = false; masterPassword = ''">Cancel</button>
        <button class="px-4 py-2 bg-green-600 text-white rounded hover:bg-green-700" @click="handleExport();  masterPassword = ''">Export</button>
      </div>
    </template>
  </Modal>

  <!-- Import Modal -->
  <Modal :show="showImportModal">
    <template #header>
      <h2 class="text-xl font-bold mb-4">Import new Passwords</h2>
    </template>

    <template #body>
      <textarea v-model="importJSON" class="w-full h-16 rounded text-sm p-1 border border-neutral-600" placeholder="Credentials as JSON"></textarea>
      <input
        type="password"
        v-model="masterPassword"
        class="w-full px-2 py-1 border rounded mb-4 border border-neutral-600"
        placeholder="Master Password"
      />
    </template>

    <template #footer>
      <div class="flex justify-end gap-2">
        <button class="px-4 py-2 bg-neutral-600 rounded hover:bg-neutral-700" @click="showImportModal = false; masterPassword = ''">Cancel</button>
        <button class="px-4 py-2 bg-green-600 text-white rounded hover:bg-green-700" @click="handleImport(); masterPassword = ''">Import</button>
      </div>
    </template>
  </Modal>
</template>
