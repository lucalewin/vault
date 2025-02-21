<script lang="ts">
  let username = $state('');
  let email = $state('');
  let password = $state('');
  let error = $state('');
  let success = $state('');

  async function handleSubmit(event: SubmitEvent) {
    event.preventDefault();
    error = "";
    success = "";
    try {
      const res = await fetch("http://localhost:8000/api/v1/auth/register", {
        method: "POST",
        headers: {
          "Content-Type": "application/json"
        },
        body: JSON.stringify({ username, email, password })
      });
      const data = await res.json();
      if (!res.ok) {
        error = data.message || "Registration failed.";
      } else {
        success = data.message || "Registration successful!";
      }
    } catch (err) {
      console.log(err);
      error = "An error occurred while registering.";
    }
  }
</script>

<form onsubmit={handleSubmit} class="flex flex-col gap-4">
  <h1>Register</h1>
  <div>
    <label for="username">Username:</label>
    <input id="username" type="text" bind:value={username} required />
  </div>
  <div>
    <label for="email">Email:</label>
    <input id="email" type="email" bind:value={email} required />
  </div>
  <div>
    <label for="password">Password:</label>
    <input id="password" type="password" bind:value={password} required />
  </div>
  <button type="submit">Register</button>
</form>

{#if error}
    <p style="color: red;">{error}</p>
{/if}

{#if success}
    <p style="color: green;">{success}</p>
{/if}