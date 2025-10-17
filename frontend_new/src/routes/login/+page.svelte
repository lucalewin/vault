<script>
  import { goto } from "$app/navigation";
  import { Button } from "$lib/components/ui/button";
  import * as Card from "$lib/components/ui/card";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import { token } from "$lib/stores/authStore";

  let email = $state("");
  let password = $state("");

  const handleLogin = async () => {
    try {
      const response = await fetch("/api/v1/auth/login", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ email, password }),
      });

      if (!response.ok) {
        throw new Error("Login failed");
      }

      const data = await response.json();
      token.set(data.token);
      // console.log(appState);
    } catch (error) {
      console.error(error);
      throw error;
    }
    console.log("Login successful");
    await goto("/vault");
    console.log("Redirected to vault");
  };
</script>

<div class="min-h-screen flex justify-center items-center">
  <Card.Root>
    <Card.Header>
      <Card.Title class="text-2xl font-bold">Login</Card.Title>
      <Card.Description>
        Please enter your credentials to access the system.
      </Card.Description>
    </Card.Header>
    <Card.Content>
      <form class="min-w-sm w-full flex flex-col gap-4" onsubmit={handleLogin}>
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
        <Button type="submit">Login</Button>
      </form>
    </Card.Content>
  </Card.Root>
</div>
