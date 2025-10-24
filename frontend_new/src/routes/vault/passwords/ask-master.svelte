<script lang="ts">
  import { Button, buttonVariants } from "$lib/components/ui/button";
  import * as Drawer from "$lib/components/ui/drawer";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import { token } from "$lib/stores/authStore";
  import type { PwCredential } from "./columns";
  import * as Password from "$lib/components/ui/password";

  let {
    open = $bindable(false),
    id = $bindable(""),
  }: { open?: boolean; id: string } = $props();

  let masterPassword = $state("");
  let credential = $state(null as PwCredential | null);

  $effect(() => {
    if (!open) {
      credential = null;
      masterPassword = "";
    }
  });

  const fetchCredential = async () => {
    try {
      const response = await fetch(`/api/v1/credentials/${id}`, {
        method: "POST",
        headers: {
          Authorization: `Bearer ${$token}`,
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ master_password: masterPassword }),
      });
      let json = await response.json();
      credential = json.credential;
    } catch (error) {
      console.error(error);
      credential = null;
    }
  };
</script>

<Drawer.Root bind:open>
  <Drawer.Content>
    {#if credential !== null}
      <Drawer.Header class="text-left">
        <Drawer.Title>{credential.service}</Drawer.Title>
      </Drawer.Header>
      <div class="grid items-start gap-4 px-4">
        <div class="grid gap-2">
          <Label for="username">Username:</Label>
          <Input readonly disabled value={credential.username} type="text" />
        </div>
        <div class="grid gap-2">
          <Label for="password">Password:</Label>
          <Password.Root>
            <Password.Input readonly disabled value={credential.password ?? ""}>
              <Password.Copy />
              <Password.ToggleVisibility />
            </Password.Input>
          </Password.Root>
        </div>
      </div>
      <Drawer.Footer class="pt-2">
        <Drawer.Close class={buttonVariants({ variant: "outline" })}
          >Close</Drawer.Close
        >
      </Drawer.Footer>
    {:else}
      <Drawer.Header class="text-left">
        <Drawer.Title>Enter Master Password</Drawer.Title>
      </Drawer.Header>
      <form class="grid items-start gap-4 px-4" onsubmit={fetchCredential}>
        <div class="grid gap-2">
          <Input type="password" id="password" bind:value={masterPassword} />
        </div>
      </form>
      <Drawer.Footer class="pt-2">
        <Button type="submit">Verify</Button>
      </Drawer.Footer>
    {/if}
  </Drawer.Content>
</Drawer.Root>
