<script>
  import * as Card from "$lib/components/ui/card";
</script>

<div class="max-w-4xl w-full mx-auto p-4">
  <Card.Root>
    <Card.Header>
      <Card.Title>Vault</Card.Title>
      <Card.Description>Manage your passwords securely.</Card.Description>
    </Card.Header>
    <Card.Content class="flex flex-col">
      <a href="/vault/passwords">View Passwords</a>
      <a href="/vault/authenticator">View Authenticator</a>
    </Card.Content>
  </Card.Root>
</div>
