<form on:submit|preventDefault={addCredential} class="mx-auto max-w-md">
  <h2>Add New Credential</h2>
  <label>
    Service:
    <input type="text" bind:value={newCredential.service} required />
  </label>
  <label>
    Username:
    <input type="text" bind:value={newCredential.username} required />
  </label>
  <label>
    Password:
    <input type="password" bind:value={newCredential.password} required />
  </label>
  <label>
    Master Password:
    <input type="password" bind:value={newCredential.master_password} required />
  </label>
  <button type="submit">Add Credential</button>
</form>

{#if responseMessage}
<div class="mt-4 whitespace-pre-wrap">
  <h3>Response:</h3>
  <pre>{responseMessage}</pre>
</div>
{/if}

<script lang="ts">
  let newCredential = {
    master_password: '',
    service: '',
    username: '',
    password: ''
  };
  let responseMessage = '';

  async function addCredential() {
    const apiToken = localStorage.getItem('apiToken');
    if (apiToken) {
      try {
        const response = await fetch('http://localhost:8000/api/v1/credentials', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
            'Authorization': `Bearer ${apiToken}`
          },
          body: JSON.stringify(newCredential)
        });
        responseMessage = response.ok ? 'Credential added successfully!' : 'Failed to add credential';
        if (response.ok) {

          // const addedCredential = await response.json();
          // if (!credentials[addedCredential.service]) {
          //   credentials[addedCredential.service] = [];
          // }
          // credentials[addedCredential.service].push(addedCredential);
          // newCredential = { service: '', username: '', password: '' };
        } else {
          console.error('Failed to add credential');
        }
      } catch (error) {
        console.error('Error:', error);
      }
    }
  }
</script>

<style>
  form {
    margin-top: 2em;
  }

  label {
    display: block;
    margin: 0.5em 0;
  }

  input {
    width: 100%;
    padding: 0.5em;
    margin-top: 0.2em;
    border: 1px solid #ccc;
    border-radius: 5px;
  }

  button {
    display: inline-block;
    margin-top: 1em;
    padding: 0.5em 1em;
    color: white;
    background-color: #3498db;
    text-decoration: none;
    border: none;
    border-radius: 5px;
    cursor: pointer;
  }

  button:hover {
    background-color: #2980b9;
  }
</style>