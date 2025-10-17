<script>
  import * as Card from "$lib/components/ui/card";
  import { vault } from "$lib/stores/credentialStore";
  import ShieldCheckIcon from "@lucide/svelte/icons/shield-check";
  import ShieldAlertIcon from "@lucide/svelte/icons/shield-alert";
</script>

<div class="max-w-4xl w-full mx-auto p-4">
  <Card.Root>
    <Card.Header>
      <Card.Title>Vault</Card.Title>
      <Card.Description>Manage your passwords securely.</Card.Description>
    </Card.Header>
    <Card.Content>
      <h2>{$vault?.credentials?.length}</h2>
      <p>Number of Passwords</p>
    </Card.Content>
  </Card.Root>
  <Card.Root>
    <Card.Header>
      <Card.Title>Security</Card.Title>
      <Card.Description></Card.Description>
    </Card.Header>
    <Card.Content>
      <ShieldCheckIcon size="128" strokeWidth={1.5} color="green" />
      <ShieldAlertIcon size="128" strokeWidth={1.5} color="red" />
    </Card.Content>
  </Card.Root>
</div>
