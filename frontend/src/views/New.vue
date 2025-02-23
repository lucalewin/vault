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
            <span @click="generatePassword"><i class="pi pi-refresh"></i></span>
            <span @click="copyPassword"><i class="pi pi-copy"></i></span>
            <span @click="toggleShowPassword"><i :class="showPassword ? 'pi pi-eye-slash': 'pi pi-eye'"></i></span>
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

const router = useRouter();
const service = ref('');
const username = ref('');
const password = ref('');
const master_password = ref('');
const showPassword = ref(false);
const loading = ref(false);
const error = ref('');

const generatePassword = () => {
  const upper = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ';
  const lower = 'abcdefghijklmnopqrstuvwxyz';
  const numbers = '0123456789';
  const symbols = '!@#$%^&*()_+[]{}|;:,.<>?';
  const allChars = upper + lower + numbers + symbols;

  let generatedPassword = '';
  generatedPassword += upper.charAt(Math.floor(Math.random() * upper.length));
  generatedPassword += lower.charAt(Math.floor(Math.random() * lower.length));
  generatedPassword += numbers.charAt(Math.floor(Math.random() * numbers.length));
  generatedPassword += symbols.charAt(Math.floor(Math.random() * symbols.length));

  for (let i = 4; i < 25; i++) {
    generatedPassword += allChars.charAt(Math.floor(Math.random() * allChars.length));
  }

  password.value = generatedPassword.split('').sort(() => 0.5 - Math.random()).join('');
};

const copyPassword = () => {
  if (navigator.clipboard) {
    navigator.clipboard.writeText(password.value).then(() => {
      // alert('Password copied to clipboard! 234234234');
    }).catch(err => {
      console.error('Failed to copy password:', err);
    });
  } else {
    const textArea = document.createElement('textarea');
    textArea.value = password.value;
    document.body.appendChild(textArea);
    textArea.select();
    try {
      document.execCommand('copy');
    } catch (err) {
      console.error('Failed to copy password:', err);
    }
    document.body.removeChild(textArea);
  }
};

const toggleShowPassword = () => {
  showPassword.value = !showPassword.value;
};

const handleSubmit = async () => {
  loading.value = true;
  try {
    const token = localStorage.getItem('api_token');
    const response = await fetch('/api/v1/credentials', {
      method: 'POST',
      headers: {
        'Authorization': `Bearer ${token}`,
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        service: service.value,
        username: username.value,
        password: password.value,
        master_password: master_password.value,
      }),
    });

    if (!response.ok) {
      throw new Error('Network response was not ok');
    }

    // check if json message contains status=sucess
    const data = await response.json();
    if (data.status !== 'success') {
      // throw new Error('Failed to save credential');
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
  border: 1px solid #ccc;
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
  /* gap: 0.5rem; */
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