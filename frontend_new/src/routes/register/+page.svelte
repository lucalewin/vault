<script>
  import { Button } from "$lib/components/ui/button";
  import * as Card from "$lib/components/ui/card";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";

  import { token } from "$lib/stores/authStore";

  let email = $state("");
  let password = $state("");

  export const register = async () => {
    try {
      const response = await fetch("/api/v1/auth/register", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ email, password, username: email }),
      });

      if (!response.ok) {
        throw new Error("Registration failed");
      }

      const data = await response.json();
      token.set(data.token);
      console.log(data);
    } catch (error) {
      console.error(error);
      throw error;
    }
  };
</script>

<div class="min-h-screen flex justify-center items-center">
  <Card.Root>
    <Card.Header>
      <Card.Title class="text-2xl font-bold">Register</Card.Title>
      <Card.Description>
        Please enter your credentials to create an account.
      </Card.Description>
    </Card.Header>
    <Card.Content>
      <form class="min-w-sm w-full flex flex-col gap-4">
        <div class="flex flex-col gap-2">
          <Label for="email">Email</Label>
          <Input
            id="email"
            type="email"
            placeholder="Email"
            bind:value={email}
          />
        </div>
        <div class="flex flex-col gap-2">
          <Label for="password">Password</Label>
          <Input
            id="password"
            type="password"
            placeholder="Password"
            bind:value={password}
          />
        </div>
        <Button onclick={register}>Register</Button>
      </form>
    </Card.Content>
  </Card.Root>
</div>
