<template>
  <div class="register">
    <h1>Register</h1>
    <form v-if="!recoveryPhrase" @submit.prevent="register">
      <div>
        <label for="username">Username:</label>
        <input type="text" id="username" v-model="username" required />
      </div>
      <div>
        <label for="email">Email:</label>
        <input type="email" id="email" v-model="email" required />
      </div>
      <div>
        <label for="password">Password:</label>
        <input type="password" id="password" v-model="password" required />
      </div>
      <button type="submit">Register</button>
      <LinearSpinner :loading="loading" />
      <p v-if="error" style="color: red;">{{ error }}</p>
    </form>
    <div v-else>
      <p>Registration successful! Please save your recovery phrase securely:</p>
      <div class="input-container">
        <input type="text" :value="recoveryPhrase" class="input" readonly>
        <span class="input-icons">
          <span @click="copyToClipboard(recoveryPhrase)"><i class="pi pi-copy"></i></span>
        </span>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import LinearSpinner from '../components/LinearSpinner.vue';
import { copyToClipboard } from '../lib/util.ts';

const router = useRouter();
const username = ref('');
const email = ref('');
const password = ref('');
const error = ref('');
const recoveryPhrase = ref('');
const loading = ref(false);

const register = async () => {
  loading.value = true;
  try {
    const response = await fetch('/api/v1/auth/register', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        username: username.value,
        email: email.value,
        password: password.value
      })
    });

    if (!response.ok) {
      throw new Error('Registration failed');
    }
    
    const data = await response.json();
    if (data.status === 'success') {
      console.log('Registration successful:', data);
      recoveryPhrase.value = data.recovery_phrase;
    } else {
      console.error('Registration failed:', data.message);
      error.value = data.message;
    }
  } catch (error) {
    console.error('Registration failed:', error);
  } finally {
    loading.value = false;
  }
};

const copyRecoveryPhrase = () => {
  if (navigator.clipboard) {
    navigator.clipboard.writeText(recoveryPhrase.value).then(() => {
      alert('Recovery phrase copied to clipboard');
    }).catch(err => {
      console.error('Failed to copy:', err);
    });
  } else {
    const textArea = document.createElement('textarea');
    textArea.value = recoveryPhrase.value;
    textArea.style.position = 'fixed';  // Avoid scrolling to bottom
    textArea.style.opacity = '0';
    document.body.appendChild(textArea);
    textArea.focus();
    textArea.select();
    try {
      document.execCommand('copy');
      alert('Recovery phrase copied to clipboard');
    } catch (err) {
      console.error('Failed to copy:', err);
    }
    document.body.removeChild(textArea);
  }
};
</script>

<style scoped>
/* Add styles similar to the login page */
.register {
  max-width: 400px;
  margin: 0 auto;
  padding: 1em;
  border: 1px solid #ccc;
  border-radius: 4px;
}
.register h1 {
  text-align: center;
}
.register div {
  margin-bottom: 1em;
}
.register label {
  display: block;
  margin-bottom: 0.5em;
}
.register input {
  width: 100%;
  padding: 0.5em;
  box-sizing: border-box;
  color: #000; /* Ensure text color is black */
  background-color: #fff; /* Ensure background color is white */
}
.register button {
  width: 100%;
  padding: 0.7em;
  background-color: #007bff;
  border: none;
  color: white;
  cursor: pointer;
}
.register button:hover {
  background-color: #0056b3;
}
.recovery-phrase {
  display: flex;
  align-items: center;
  gap: 0.5em;
}
/* .recovery-phrase input {
  flex: 1;
  padding: 0.5em;
  box-sizing: border-box;
  color: #000; 
  background-color: #fff; 
} */
.recovery-phrase button {
  padding: 0.5em 1em;
  background-color: #007bff;
  border: none;
  color: white;
  cursor: pointer;
}
.recovery-phrase button:hover {
  background-color: #0056b3;
}

.input-container {
  display: flex;
  align-items: center;
  position: relative;
}

.input-container input {
  flex: 1;
  padding-right: 48px;
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

</style>