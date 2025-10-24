<script>
  import { Button } from "$lib/components/ui/button";
  import * as Card from "$lib/components/ui/card";
  import * as Empty from "$lib/components/ui/empty";
  import ArrowUpRightIcon from "@lucide/svelte/icons/arrow-up-right";
  import LockIcon from "@lucide/svelte/icons/lock";
  import { resource, useInterval } from "runed";
  import { UseClipboard } from "$lib/hooks/use-clipboard.svelte";
  import { toast } from "svelte-sonner";
  import { Toaster } from "$lib/components/ui/sonner";
  import { token } from "$lib/stores/authStore";
  import { IsMobile } from "$lib/hooks/is-mobile.svelte";
  import { MediaQuery } from "svelte/reactivity";
  import CodeList from "./code-list.svelte";

  const recalculateDelay = () => 30 - (Math.floor(Date.now() / 1000) % 30);

  const authCodes = resource(
    () => {},
    async () => {
      const resp = await fetch("/api/v1/authenticator", {
        method: "GET",
        headers: {
          Authorization: `Bearer ${$token}`,
        },
      });
      return resp.json();
    },
  );

  let empty = $derived((authCodes.current ?? []).length === 0);
  let delay = $state(recalculateDelay());

  useInterval(() => delay * 1000, {
    callback: async () => {
      delay = recalculateDelay();
      await authCodes.refetch();
    },
  });

  const clipboard = new UseClipboard();
  const isDesktop = new MediaQuery("(min-width: 768px)");
</script>

<!-- todo: add empty when no authentication code are provided -->

<Toaster />

<div class="max-w-4xl w-full mx-auto">
  {#if !empty}
    {#if isDesktop.current}
      <Card.Root>
        <Card.Header>
          <Card.Title>Authenticator</Card.Title>
        </Card.Header>
        <Card.Content>
          <CodeList bind:authCodes={authCodes.current} />
          <!-- <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            {#each authCodes.current as auth}
              <button
                type="button"
                class="text-start border rounded-lg p-4 hover:cursor-pointer hover:bg-background/40"
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
          </div> -->
        </Card.Content>
      </Card.Root>
    {:else}
      <CodeList bind:authCodes={authCodes.current} />
    {/if}
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
