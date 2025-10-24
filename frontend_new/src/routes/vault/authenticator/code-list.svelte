<script>
  import { UseClipboard } from "$lib/hooks/use-clipboard.svelte";
  import { toast } from "svelte-sonner";

  let { authCodes = $bindable() } = $props();

  const clipboard = new UseClipboard();
</script>

<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
  {#each authCodes as auth}
    <button
      type="button"
      class="text-start border rounded-lg p-4 hover:cursor-pointer hover:bg-background/40 bg-card sm:bg-background"
      onclick={() => {
        clipboard.copy(auth.code);
        toast.success("Copied to clipboard!");
      }}
    >
      <div>
        <span class="font-bold text-lg mr-1">{auth.service}</span>
        <span class="text-muted-foreground text-sm">
          {auth.username}
        </span>
      </div>
      <span class="text-primary text-4xl font-semibold">
        {auth.code.substring(0, 3)}
        {auth.code.substring(3)}
      </span>
    </button>
  {/each}
</div>
