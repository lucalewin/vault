<script lang="ts">
  import { Button, buttonVariants } from "$lib/components/ui/button";
  import * as Dialog from "$lib/components/ui/dialog/index.js";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import * as Password from "$lib/components/ui/password";
  import { token } from "$lib/stores/authStore";

  let open = $state(false);

  let name = $state("");
  let username = $state("");
  let password = $state("");
  let masterPassword = $state("");

  let showPassword = $state(false);
  let showMasterPassword = $state(false);

  const handleCreatePassword = async () => {
    // Handle password creation logic here
    console.log($token);
    try {
      const response = await fetch("/api/v1/credentials", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${$token}`,
        },
        body: JSON.stringify({
          service: name,
          username,
          password,
          master_password: masterPassword,
        }),
      });

      console.log(response);

      // TODO: Handle response
      if (response.ok) {
      } else {
      }
    } catch (error) {
      // Handle error
      console.error(error);
    }

    open = false;
  };
</script>

<Dialog.Root bind:open>
  <Dialog.Trigger class={buttonVariants({ variant: "default" })}>
    Create Password
  </Dialog.Trigger>
  <Dialog.Content class="sm:max-w-[425px]">
    <Dialog.Header>
      <Dialog.Title>Create Password</Dialog.Title>
      <Dialog.Description>
        Create a new password for your vault.
      </Dialog.Description>
    </Dialog.Header>
    <div class="grid gap-4 py-4">
      <div class="flex flex-col gap-2">
        <Label for="name" class="text-right">Name</Label>
        <Input id="name" bind:value={name} class="col-span-3" />
      </div>
      <div class="flex flex-col gap-2">
        <Label for="username" class="text-right">Username</Label>
        <Input id="username" bind:value={username} class="col-span-3" />
      </div>
      <div class="flex flex-col gap-2">
        <Label for="password" class="text-right">Password</Label>
        <Password.Root>
          <Password.Input bind:value={password}>
            <Password.Generate bind:text={password} />
            <Password.ToggleVisibility />
          </Password.Input>
        </Password.Root>
      </div>
      <div class="flex flex-col gap-2">
        <Label for="master-password" class="text-right">Master Password</Label>
        <Password.Root>
          <Password.Input bind:value={masterPassword}>
            <Password.ToggleVisibility />
          </Password.Input>
        </Password.Root>
      </div>
      <Dialog.Footer>
        <Button variant="outline" onclick={() => (open = false)}>Cancel</Button>
        <Button type="submit" onclick={handleCreatePassword}
          >Create Password</Button
        >
      </Dialog.Footer>
    </div></Dialog.Content
  >
</Dialog.Root>
