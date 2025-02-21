<script lang="ts">
  import { onMount } from 'svelte';

  let credentials: any = {};

  onMount(async () => {
      const apiToken = localStorage.getItem('apiToken');
      if (apiToken) {
          try {
              const response = await fetch('http://localhost:8000/api/v1/credentials', {
                  headers: {
                      'Authorization': `Bearer ${apiToken}`
                  }
              });
              if (response.ok) {
                  const data = await response.json();
                  credentials = data.credentials.reduce((acc: any, credential: any) => {
                      if (!acc[credential.service]) {
                          acc[credential.service] = [];
                      }
                      acc[credential.service].push(credential);
                      return acc;
                  }, {});
              } else {
                  console.error('Failed to fetch credentials');
              }
          } catch (error) {
              console.error('Error:', error);
          }
      }
  });


  // onMount(async () => {
  //   const apiToken = localStorage.getItem('apiToken');
  //   if (apiToken) {
  //     try {
  //       const response = await fetch('http://localhost:8000/api/v1/credentials', {
  //         headers: {
  //           'Authorization': `Bearer ${apiToken}`
  //         }
  //       });
  //       if (response.ok) {
  //         const data = await response.json();
  //         credentials = data.credentials.reduce((acc: any, credential: any) => {
  //           if (!acc[credential.service]) {
  //             acc[credential.service] = [];
  //           }
  //           acc[credential.service].push(credential);
  //           return acc;
  //         }, {});
  //       } else {
  //         console.error('Failed to fetch credentials');
  //       }
  //     } catch (error) {
  //       console.error('Error:', error);
  //     }
  //   }
  // });

  async function promptForPassword(credentialId: string) {
    const masterPassword = prompt('Enter your master password:');
    if (masterPassword) {
      try {
        const apiToken = localStorage.getItem('apiToken');
        const response = await fetch(`http://localhost:8000/api/v1/credentials/${credentialId}`, {
          method: 'POST',
          headers: {
            'Authorization': `Bearer ${apiToken}`,
            'Content-Type': 'application/json'
          },
          body: JSON.stringify({ "master_password": masterPassword })
        });
        if (response.ok) {
          const data = await response.json();
          alert(`Password: ${data.credential}`);
          // const credential = credentials.find((cred: any) => cred.id === credentialId);
          // if (credential) {
          //   credential.password = data.password;
          //   credential.showPassword = true;
          // }
        } else {
          console.error('Failed to fetch password');
        }
      } catch (error) {
        console.error('Error:', error);
      }
    }
  }
</script>

<a href="/register">Register</a>
<a href="/login">Login</a>
<a href="/new">New</a>

<div class="max-w-xl mx-auto">
    <h1>Credentials</h1>
    <p>Here are your credentials:</p>

    {#if Object.keys(credentials).length > 0}
      {#each Object.keys(credentials) as service}
          <h2>{service}</h2>
          <ul>
              {#each credentials[service] as credential}
                <li>
                  {credential.username}
                  <button on:click={() => promptForPassword(credential.id)}>View Password</button>
                  {#if credential.showPassword}
                    <p>{credential.password}</p>
                  {/if}
                </li>
              {/each}
          </ul>
      {/each}
    {/if}
</div>


<style>
  h2 {
      color: #2c3e50;
      font-size: 1.5em;
      margin-top: 1em;
  }

  ul {
      list-style-type: none;
      padding: 0;
  }

  li {
      background-color: #ecf0f1;
      margin: 0.5em 0;
      padding: 0.5em;
      border-radius: 5px;
  }

  a {
      display: inline-block;
      margin: 0.5em 0;
      padding: 0.5em 1em;
      color: white;
      background-color: #3498db;
      text-decoration: none;
      border-radius: 5px;
  }

  a:hover {
      background-color: #2980b9;
  }
</style>
