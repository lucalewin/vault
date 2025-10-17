<script lang="ts">
  import ModeSwitcher from "$lib/components/mode-switcher.svelte";
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import { ModeWatcher } from "mode-watcher";
  import VaultSidebar from "./sidebar.svelte";

  let { children } = $props();
</script>

<ModeWatcher />
<Sidebar.Provider>
  <VaultSidebar />
  <main class="w-full">
    <!-- <header class="p-4 border-b flex justify-between items-center mb-4"> -->
    <header
      class="bg-background sticky top-0 flex h-16 shrink-0 items-center gap-2 border-b px-4 z-1000"
    >
      <Sidebar.Trigger />
      <div>
        <ModeSwitcher />
      </div>
    </header>
    <div class="p-4">
      {@render children?.()}
    </div>
  </main>
</Sidebar.Provider>
