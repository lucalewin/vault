<template>
  <form @submit.prevent="handleSubmit">
    <h1>New credential</h1>
    <div>
      <label for="service">Service</label>
      <input type="text" id="service" v-model="service" class="input" required>
    </div>
    <div>
      <label for="username">Username</label>
      <input type="text" id="username" v-model="username" class="input" required>
    </div>
    <div class="password-container">
      <label for="password">Password</label>
      <div class="password-input">
        <span class="password-container">
          <input :type="showPassword ? 'text' : 'password'" id="password" v-model="password" class="input" required>
          <span class="input-icons">
            <span @click="password = generatePassword()"><i class="pi pi-refresh"></i></span>
            <span @click="copyToClipboard(password)"><i class="pi pi-copy"></i></span>
            <span @click="showPassword = !showPassword"><i :class="showPassword ? 'pi pi-eye': 'pi pi-eye-slash'"></i></span>
          </span>
        </span>
      </div>
    </div>
    <div>
      <label for="master_password">Master Password</label>
      <input type="password" id="master_password" v-model="master_password" class="input" required>
    </div>
    <button type="submit" class="btn">Save</button>
    <LinearSpinner :loading="loading" />
    <p v-if="error" style="color: red;">{{ error }}</p>
  </form>
</template>

<script setup>
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import LinearSpinner from '../components/LinearSpinner.vue';
import { copyToClipboard, generatePassword } from '../lib/util';
import { new_credential } from '../lib/api/credentials';

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
form {
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
}

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