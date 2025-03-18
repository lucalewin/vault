<template>
  <Modal :show="show">
    <template #header>
      <h2 class="text-xl font-bold mb-4">Import new Passwords</h2>
    </template>

    <template #body>
      <textarea v-model="jsonText" class="w-full h-16 rounded text-sm p-1 border border-neutral-600" placeholder="Credentials as JSON"></textarea>
      <input
        type="password"
        v-model="masterPassword"
        class="w-full px-2 py-1 border rounded mb-4 border border-neutral-600"
        placeholder="Master Password"
      />
    </template>

    <template #footer>
      <div class="flex justify-end gap-2">
        <button class="px-4 py-2 bg-neutral-600 rounded hover:bg-neutral-700" @click="masterPassword = ''">Cancel</button>
        <button class="px-4 py-2 bg-green-600 text-white rounded hover:bg-green-700" @click="masterPassword = ''">Import</button>
      </div>
    </template>
  </Modal>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import Modal from '../../components/Modal.vue';

defineProps({
  show: Boolean
})

const jsonText = ref('');
const masterPassword = ref('');
</script>