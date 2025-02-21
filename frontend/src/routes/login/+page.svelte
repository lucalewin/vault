<script lang="ts">
  import { goto } from '$app/navigation';

  let email = '';
  let password = '';
  let responseMessage = '';
  let isError = false;

  async function handleSubmit(event: SubmitEvent) {
    event.preventDefault();

    const response = await fetch('http://localhost:8000/api/v1/auth/login', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({ email, password })
    });

    const data = await response.json();
    
    if (data.status === 'success') {
      localStorage.setItem('apiToken', data.token);
      responseMessage = 'Login successful!';
      goto("/");
      isError = false;
    } else {
      responseMessage = data.message || 'An error occurred';
      isError = true;
    }
  }
</script>

<div class="mx-auto p-4 max-w-md">
  <form on:submit|preventDefault={handleSubmit}>
    <div class="mb-4">
      <label for="email">Email</label>
      <input id="email" type="email" bind:value={email} class="w-full p-2 border border-gray-300 rounded" required />
    </div>
    <div class="mb-4">
      <label for="password">Password</label>
      <input id="password" type="password" bind:value={password} class="w-full p-2 border border-gray-300 rounded" required />
    </div>
    <button type="submit" class="w-full py-2 bg-blue-500 text-white rounded">Login</button>
  </form>

  {#if responseMessage}
    <div class="mt-4 whitespace-pre-wrap">
      <h3>Response:</h3>
      <pre>{responseMessage}</pre>
    </div>
  {/if}
</div>