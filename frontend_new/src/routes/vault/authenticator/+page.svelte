<script>
  import { Button } from "$lib/components/ui/button";
  import * as Card from "$lib/components/ui/card";
  import * as Empty from "$lib/components/ui/empty";
  import { authenticator } from "$lib/state";
  import { token } from "$lib/stores/authStore";
  import { vault } from "$lib/stores/credentialStore";
  import ArrowUpRightIcon from "@lucide/svelte/icons/arrow-up-right";
  import LockIcon from "@lucide/svelte/icons/lock";
  import { onMount } from "svelte";

  import EditIcon from "@lucide/svelte/icons/pencil";
  import TrashIcon from "@lucide/svelte/icons/trash";
  import { fetchAuthenticator } from "$lib";
  import { useInterval } from "runed";

  const recalculateDelay = () => 30 - (Math.floor(Date.now() / 1000) % 30);

  let empty = $derived(authenticator.current.length === 0);
  let delay = $state(recalculateDelay());

  useInterval(() => delay * 1000, {
    callback: async () => {
      await fetchAuthenticator();
      delay = recalculateDelay();
    },
  });

  onMount(async () => {
    await fetchAuthenticator();
  });
</script>

<!-- todo: add empty when no authentication code are provided -->

<div class="max-w-4xl w-full mx-auto">
  {#if !empty}
    <Card.Root>
      <Card.Header>
        <Card.Title>Authenticator</Card.Title>
      </Card.Header>
      <Card.Content>
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          {#each authenticator.current as auth}
            <div class="border rounded p-4">
              {auth.service}
              {auth.username}
              {auth.code}
            </div>
          {/each}
        </div>
      </Card.Content>
    </Card.Root>
  {:else}
    <Empty.Root>
      <Empty.Header>
        <Empty.Media variant="icon">
          <LockIcon />
        </Empty.Media>
        <Empty.Title>No Authenticator Yet</Empty.Title>
        <Empty.Description>
          You haven't created any authenticators yet. Get started by creating
          your first authenticator.
        </Empty.Description>
      </Empty.Header>
      <Empty.Content>
        <div class="flex gap-2">
          <Button disabled>Scan QR Code</Button>
          <!-- <CreateModal /> -->
          <Button variant="outline" disabled>Import Authenticator</Button>
        </div>
      </Empty.Content>
      <Button variant="link" class="text-muted-foreground" size="sm">
        <a href="#/">
          Learn More <ArrowUpRightIcon class="inline" />
        </a>
      </Button>
    </Empty.Root>
  {/if}
</div>
