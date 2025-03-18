<template>
  <Modal :show="showModal">
    <template #header>
      <h2 class="text-xl font-bold mb-4">Add a new Code</h2>
    </template>

    <template #body>
      <input
        type="password"
        v-model="service"
        class="w-full px-2 py-1 border rounded mb-4 border border-neutral-600"
        placeholder="Service"
      />
      <input
        type="password"
        v-model="username"
        class="w-full px-2 py-1 border rounded mb-4 border border-neutral-600"
        placeholder="Username"
      />
      <input
        type="password"
        v-model="secret"
        class="w-full px-2 py-1 border rounded mb-4 border border-neutral-600"
        placeholder="Secret"
      />
    </template>

    <template #footer>
      <div class="flex justify-end gap-2">
        <button class="px-4 py-2 bg-neutral-600 rounded hover:bg-neutral-700" @click="$emit('close')">Cancel</button>
        <button class="px-4 py-2 bg-green-600 text-white rounded hover:bg-green-700" @click="$emit('close'); insert_code()">Add</button>
      </div>
    </template>
  </Modal>
</template>

<script setup lang="ts">
import Modal from '@/components/Modal.vue';
import { ref } from 'vue';

const showModal = ref(true);

const service = ref('');
const username = ref('');
const secret = ref('');

async function insert_code() {
  try {
    const apiToken = localStorage.getItem('api_token')
    const response = await fetch('/api/v1/authenticator', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${apiToken}`,
      },
      body: JSON.stringify({
        service: service.value,
        username: username.value,
        secret: secret.value,
      }),
    });

    if (!response.ok) {
      throw new Error('Failed to insert code');
    }

    const result = await response.json();
    console.log('Code inserted successfully:', result);
  } catch (error) {
    console.error('Error inserting code:', error);
  }
}
</script>