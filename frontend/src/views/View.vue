<template>
  <div>
    <form v-if="!credentials.length" @submit.prevent="handleSubmit" class="form">
      <h1>Challenge for {{ service }}</h1>
      <div>
        <label for="master_password">Master Password</label>
        <input type="password" id="master_password" v-model="masterPassword" class="input" required>
      </div>
      <button type="submit" class="btn">Submit</button>
      <LinearSpinner :loading="loading" />
    </form>
    <div v-if="credentials.length">
      <h1>Credentials for {{ service }}</h1>
      <ul class="credentials-list">
        <li v-for="credential in credentials" :key="credential.id" class="credential">
          <div>
            <label for="username">Username:</label>
            <div class="input-container">
              <input type="text" id="username" :value="credential.credential.username" class="input" readonly>
              <span class="input-icons">
                <span @click="copyToClipboard(credential.credential.username)"><i class="pi pi-copy"></i></span>
              </span>
            </div>
          </div>
          <div>
            <label for="password">Password:</label>
            <div class="input-container">
              <input :type="credential.showPassword ? 'text' : 'password'" id="password" :value="credential.credential.password" class="input" readonly>
              <span class="input-icons">
                <span @click="toggleShowPassword(credential)"><i :class="credential.showPassword ? 'pi pi-eye-slash' : 'pi pi-eye'"></i></span>
                <span @click="copyToClipboard(credential.credential.password)"><i class="pi pi-copy"></i></span>
              </span>
            </div>
          </div>
        </li>
      </ul>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue';
import { useRoute } from 'vue-router';
import LinearSpinner from '../components/LinearSpinner.vue';

const route = useRoute();
const masterPassword = ref('');
const credentials = ref([]);
const loading = ref(false);
const service = ref('');

const handleSubmit = async () => {
  loading.value = true;
  const ids = route.query.ids.split(',');
  const token = localStorage.getItem('api_token');
  const fetchedCredentials = await Promise.all(ids.map(async (id) => {
    const response = await fetch(`/api/v1/credentials/${id}`, {
      method: 'POST',
      headers: {
        'Authorization': `Bearer ${token}`,
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ master_password: masterPassword.value }),
    });
    const credential = await response.json();
    credential.showPassword = false;
    return credential;
  }));
  credentials.value = fetchedCredentials;
  if (credentials.value.length > 0) {
    service.value = credentials.value[0].credential.service;
  }
  console.log(credentials.value);
  loading.value = false;
};

const copyToClipboard = (text) => {
  if (navigator.clipboard) {
    navigator.clipboard.writeText(text).then(() => {
      alert('Copied to clipboard!');
    }).catch(err => {
      console.error('Failed to copy:', err);
    });
  } else {
    const textArea = document.createElement('textarea');
    textArea.value = text;
    document.body.appendChild(textArea);
    textArea.select();
    try {
      document.execCommand('copy');
      alert('Copied to clipboard!');
    } catch (err) {
      console.error('Failed to copy:', err);
    }
    document.body.removeChild(textArea);
  }
};

const toggleShowPassword = (credential) => {
  credential.showPassword = !credential.showPassword;
};
</script>

<style scoped>
.form {
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

.credentials-list {
  list-style-type: none;
  padding: 0;
}

.credential {
  border: 1px solid #ccc;
  border-radius: 8px;
  padding: 1rem;
  margin: 0 8px;
  margin-top: 1rem;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.input-container {
  display: flex;
  align-items: center;
  position: relative;
}

.input-container input {
  flex: 1;
  padding-right: 85px;
  outline: none;
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
}

@media screen and (max-width: 440px) {
  form {
    border: none;
  }
}
</style>
