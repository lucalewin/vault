<script lang="ts">
  import { isValidDomain } from "$lib";
  import * as Avatar from "$lib/components/ui/avatar";

  let { url }: { url: string } = $props();
</script>

<Avatar.Root class="rounded-none">
  {#if isValidDomain(url)}
    <Avatar.Image src="https://{url}/favicon.ico" alt="{url} icon" />
  {/if}
  <Avatar.Fallback>{url.substring(0, 2).toUpperCase()}</Avatar.Fallback>
</Avatar.Root>
