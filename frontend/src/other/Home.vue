<!-- <template>
  Home
</template>

 -->

<!-- <template>
  <div class="container">
    <Navbar />
    <div class="content">
      <h1>Welcome to the Frontend</h1>
      <div v-if="isLoggedIn">
        <button @click="fetchCredentials">Fetch Credentials</button>
        <div id="credentials-container">
          <div v-for="(creds, service) in groupedCredentials" :key="service" class="service-group">
            <h2>{{ service }}</h2>
            <ul>
              <li v-for="cred in creds" :key="cred.username" @click="viewCredential(cred)">
                {{ cred.username }}
              </li>
            </ul>
          </div>
        </div>
      </div>
      <div v-else>
        <p>Please log in to view your credentials.</p>
      </div>
    </div>
  </div>
</template>

<script>
import { onMounted, ref, computed } from 'vue';
import { useRouter } from 'vue-router';
import Navbar from './Navbar.vue';

export default {
  components: {
    Navbar
  },
  setup() {
    const credentials = ref([]);
    const isLoggedIn = ref(false);
    const router = useRouter();

    onMounted(() => {
      const token = localStorage.getItem('api_token');
      if (token) {
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

    const viewCredential = (cred) => {
      router.push({ name: 'Challenge', params: { id: cred.id } });
    };

    return {
      isLoggedIn,
      fetchCredentials,
      groupedCredentials,
      viewCredential
    };
  }
};
</script>

<style scoped>
.container {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 20px;
  max-width: 700px;
  margin: 0 auto;
  background-color: #333;
  color: #fff;
  box-shadow: 0 0 10px rgba(0, 0, 0, 0.5);
  border-radius: 8px;
}

.content {
  text-align: center;
  width: 100%;
}

button {
  padding: 10px 20px;
  margin: 10px;
  border: none;
  border-radius: 5px;
  background-color: #007bff;
  color: #fff;
  cursor: pointer;
}

button:hover {
  background-color: #0056b3;
}

.service-group {
  margin-top: 20px;
  text-align: left;
}

.service-group h2 {
  margin-bottom: 10px;
}

ul {
  list-style-type: none;
  padding: 0;
  background-color: #444;
  border: 1px solid #555;
  border-radius: 5px;
}

li {
  padding: 10px;
  border-bottom: 1px solid #555;
  cursor: pointer;
}

li:last-child {
  border-bottom: none;
}

li:hover {
  background-color: #555;
}
</style> -->
