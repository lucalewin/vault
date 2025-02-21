<script>

  let { id = 0 } = $props();

  import { onMount } from 'svelte';
  let username = $state('');
  let email = $state('');
  let password = $state('');

  onMount(async () => {
    // Replace with your service call
    const response = await fetch('http://localhost:8000/api/v1/');
    const data = await response.json();
    username = data.username;
    email = data.email;
    password = data.password;
  });
</script>

<main>
  <h1>User Details</h1>
  <div>
    <label for="username-label">Username:</label>
    <span id="username-label">{username}</span>
  </div>
  <div>
    <label for="email-label">Email:</label>
    <span id="email-label">{email}</span>
  </div>
  <div>
    <label for="password-label">Password:</label>
    <span id="password-label">{password}</span>
  </div>
</main>

<style>
  main {
    padding: 1rem;
  }
  div {
    margin-bottom: 1rem;
  }
  label {
    font-weight: bold;
  }
</style>