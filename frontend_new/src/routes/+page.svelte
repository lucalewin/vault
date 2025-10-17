<script>
  import Navlink from "$lib/components/navlink.svelte";
  import { Button } from "$lib/components/ui/button";
</script>

<header class="p-4 border-b mb-4">
  <div class="max-w-6xl w-full mx-auto flex justify-between items-center">
    <div class="ml-4 flex gap-8">
      <Navlink href="/" title="Home">Home</Navlink>
      <Navlink href="/features" title="Features">Features</Navlink>
      <Navlink href="/pricing" title="Pricing">Pricing</Navlink>
    </div>
    <div>
      <Button href="/login" variant="outline">Login</Button>
      <Button href="/register">Register</Button>
    </div>
  </div>
</header>
<main>
  <!-- hero section -->
  <div>
    <h1>Welcome to SvelteKit</h1>
    <p>
      Visit <a href="https://svelte.dev/docs/kit">svelte.dev/docs/kit</a> to read
      the documentation
    </p>
    <a href="/vault">View Vault</a>
  </div>
</main>
<footer></footer>
