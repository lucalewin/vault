<template>
  <h1 class="m-2 font-bold text-3xl">New Password</h1>
  <Container>
    <form @submit.prevent="handleSubmit" class="mx-auto w-full max-w-sm p-8 mt-16 space-y-4 sm:border border-gray-700 rounded shadow-md">
      <h1>New credential</h1>
      <div>
        <label for="service" class="block text-sm font-medium text-gray-300">Service</label>
        <input type="text" id="service" v-model="service" class="w-full px-3 py-2 mt-1 border border-gray-700 rounded" required>
      </div>
      <div>
        <label for="username" class="block text-sm font-medium text-gray-300">Username</label>
        <input type="text" id="username" v-model="username" class="w-full px-3 py-2 mt-1 border border-gray-700 rounded" required>
      </div>
      <div class="password-container">
        <label for="password" class="block text-sm font-medium text-gray-300">Password</label>
        <div class="password-input">
          <span class="password-container">
            <input :type="showPassword ? 'text' : 'password'" id="password" v-model="password" class="w-full px-3 py-2 mt-1 border border-gray-700 rounded" required>
            <span class="input-icons">
              <span @click="password = generatePassword()">Gen </span>
              <span @click="copyToClipboard(password)">Copy </span>
              <span @click="showPassword = !showPassword">View</span>
            </span>
          </span>
        </div>
      </div>
      <div>
        <label for="master_password" class="block text-sm font-medium text-gray-300">Master Password</label>
        <input type="password" id="master_password" v-model="master_password" class="w-full px-3 py-2 mt-1 border border-gray-700 rounded" required>
      </div>
      <div>
        <button type="submit" class="w-full px-4 py-2 font-bold text-white bg-blue-500 rounded hover:bg-blue-700">Save</button>
      </div>
      <p v-if="error" style="color: red;">{{ error }}</p>
    </form>
  </Container>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useRouter } from 'vue-router';
// import LinearSpinner from '../components/LinearSpinner.vue';
import { copyToClipboard, generatePassword } from '../lib/util';
import { new_credential } from '../lib/api/credentials';
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

.password-input input {
  flex: 1;
  padding-right: 85px;
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
  margin-left: 0.5rem;
}

@media screen and (max-width: 440px) {
  form {
    border: none;
  }
}
</style>