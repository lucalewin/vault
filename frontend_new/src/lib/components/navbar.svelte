<script>
  import Navlink from "./navlink.svelte";
  import { Button } from "./ui/button";
  import VaultIcon from "@lucide/svelte/icons/inspection-panel";
</script>

<header class="p-4 border-b mb-4">
  <div class="max-w-3/4 mx-auto flex justify-between items-center">
    <a href="/" class="flex items-center gap-2">
      <VaultIcon size="32" class="text-primary" />
      <h1 class="font-bold text-2xl">Ferrum Vault</h1>
    </a>
    <div class="ml-4 flex gap-4">
      <Navlink href="/features" title="Features">Features</Navlink>
      <Navlink href="/download" title="Download">Download</Navlink>
      <Navlink href="/pricing" title="Pricing">Pricing</Navlink>
    </div>
    <div>
      <Button href="/login" variant="outline">Login</Button>
      <Button href="/register">Start for free</Button>
    </div>
  </div>
</header>
