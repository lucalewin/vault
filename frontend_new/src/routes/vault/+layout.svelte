<script lang="ts">
  import ModeSwitcher from "$lib/components/mode-switcher.svelte";
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import { ModeWatcher } from "mode-watcher";

  import VaultSidebar from "$lib/components/app-sidebar.svelte";
  import { IsIdle } from "runed";
  import { token } from "$lib/stores/authStore";
  import { goto } from "$app/navigation";
  import { onMount } from "svelte";
  import { parseJwt } from "$lib";

  let idle = new IsIdle({
    timeout: 5 * 60 * 1000,
    events: ["mousedown", "keydown", "touchstart", "touchmove", "touchend"],
  });

  // logout when idle
  $effect(() => {
    if (idle.current) {
      $token = null;
      goto("/login");
    }
  });

  onMount(() => {
    if ($token) {
      let jwt_decoded = parseJwt($token);
      let expiration = jwt_decoded.exp * 1000;
      let timeLeft = expiration - Date.now();
      console.log(`Token expires in ${timeLeft} milliseconds`);
      setTimeout(() => {
        console.log("Token expired");
        $token = null;
        goto("/login");
      }, timeLeft);
      let minutesLeft = Math.floor(timeLeft / (1000 * 60));
      console.log(`Token expires in ${minutesLeft} minutes`);
    } else {
      goto("/login");
    }
  });

  let { children } = $props();
</script>

<Sidebar.Provider class="h-screen">
  <VaultSidebar />
  <main class="w-full">
    <header
      class="bg-background sticky top-0 flex justify-between h-16 shrink-0 items-center gap-2 border-b px-4 z-1"
    >
      <Sidebar.Trigger />
      <div>
        Idle: {idle.current ? "Yes" : "No"}
        <ModeSwitcher />
      </div>
    </header>
    <Sidebar.Inset>
      <main class="p-4">
        {@render children?.()}
      </main>
    </Sidebar.Inset>
  </main>
</Sidebar.Provider>
