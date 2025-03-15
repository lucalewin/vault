<template>
  <div class="mx-auto mt-16 w-full max-w-sm">
    <form @submit.prevent="handleSubmit" class="border border-neutral-600 bg-neutral-800 rounded-2xl p-6 space-y-4">
      <h2 class="text-2xl font-semibold">New Password</h2>
      <div>
        <label for="service" class="block text-sm font-medium text-gray-300">Service</label>
        <input type="text" id="service" v-model="service" class="w-full px-3 py-2 mt-1 border border-neutral-600 rounded" required>
      </div>
      <div>
        <label for="username" class="block text-sm font-medium text-gray-300">Username</label>
        <input type="text" id="username" v-model="username" class="w-full px-3 py-2 mt-1 border border-neutral-600 rounded" required>
      </div>
      <div class="password-container">
        <label for="password" class="block text-sm font-medium text-gray-300">Password</label>
        <div class="password-input">
          <span class="password-container">
            <input :type="showPassword ? 'text' : 'password'" id="password" v-model="password" class="w-full px-3 py-2 pr-22 mt-1 border border-neutral-600 rounded" required>
            <span class="text-sm pt-1 text-neutral-400 flex absolute right-0 h-full gap-3 mr-3 items-center z-1">
              <i class="pi pi-sync" @click="password = generatePassword()"></i>
              <i class="pi pi-copy" @click="copyToClipboard(password)"></i>
              <i class="pi pi-eye-slash" @click="showPassword = !showPassword"></i>
            </span>
          </span>
        </div>
      </div>
      <div>
        <label for="master_password" class="block text-sm font-medium text-gray-300">Master Password</label>
        <input type="password" id="master_password" v-model="master_password" class="w-full px-3 py-2 mt-1 border border-neutral-600 rounded" required>
      </div>
      <div>
        <button type="submit" class="w-full px-4 py-2 font-semibold text-white bg-green-500/60 rounded hover:bg-green-700 focus:bg-green-700">Save</button>
      </div>
      <p v-if="error" style="color: red;">{{ error }}</p>
    </form>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useRouter } from 'vue-router';
// import LinearSpinner from '../components/LinearSpinner.vue';
import { copyToClipboard, generatePassword } from '@/lib/util';
import { new_credential } from '@/lib/api/credentials';
import Container from '@/components/Container.vue';

const router = useRouter();
const service = ref('');
const username = ref('');
const password = ref('');
const master_password = ref('');
const showPassword = ref(false);
const loading = ref(false);
const error = ref('');

// const toggleShowPassword = () => {
//   showPassword.value = !showPassword.value;
// };

const handleSubmit = async () => {
  loading.value = true;
  try {
    const data = await new_credential(
      service.value,
      username.value,
      password.value,
      master_password.value
    );

    if (data.status !== 'success') {
      error.value = data.message;
    } else {
      router.push('/');
    }
  } catch (error) {
    console.error('Failed to save credential:', error);
    alert('Failed to save credential. Please try again.');
  } finally {
    loading.value = false;
  }
};
</script>

<style scoped>
/* form {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  margin: 8px;
  padding: 1rem;
  border: 1px solid #444;
  border-radius: 4px;
}

form div {
  display: flex;
  flex-direction: column;
} */

.password-container {
  display: flex;
  flex-direction: column;
}

.password-input {
  position: relative;
}

.password-container {
  position: relative;
}

/* .password-input input {
  flex: 1;
  padding-right: 85px;
} */

/* .input-icons {
  display: flex;
  position: absolute;
  align-items: center;
  right: 0;
  height: 100%;
  z-index: 1000;
  margin-right: 12px;
} */

/* .input-icons span {
  background: none;
  border: none;
  cursor: pointer;
  margin-left: 0.5rem;
} */

</style>