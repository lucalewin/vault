<template>
  <header>
    <nav>
      <!-- <button @click="router.push('/')">Home</button> -->
      <div>
        <a href="/"><h1>Password Manager</h1></a>
      </div>
      <!-- <div>
        <button @click="router.push('/login')">Login</button>
        <button @click="router.push('/register')">Register</button>
      </div> -->
    </nav>
  </header>
</template>

<script setup>
import { useRouter } from 'vue-router';
const router = useRouter();
</script>

<style scoped>
header {
  /* background-color: #333; */
  border-bottom: 1px solid #aaa;
  margin-bottom: 1em;
  color: white;
  padding: 1rem;
  box-shadow: 0 3px 10px 0px #ccc;
}
nav {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

button~button {
  margin-left: 1em;
}

a {
  text-decoration: none;
  color: black;
}
</style>