<template>
  <header>
    <nav>
      <div>
        <a href="/"><h1>Password Manager</h1></a>
      </div>
    </nav>
  </header>
</template>

<!-- <script setup>
import { useRouter } from 'vue-router';
const router = useRouter();
</script> -->

<style scoped>
header {
  background-color: #2a2a2a; /* Darker background color for the header */
  color: #e0e0e0;
  padding: 1rem;
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.3); /* More prominent shadow */
  border-bottom: 1px solid #444; /* Less bright border */
  margin-bottom: 1em;
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
  color: inherit;
}
</style>