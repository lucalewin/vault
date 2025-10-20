<script>
  import VaultIcon from "@lucide/svelte/icons/inspection-panel";
  import LinkIcon from "@lucide/svelte/icons/square-arrow-out-up-right";
  import { Button } from "./ui/button";
</script>

<section class="relative overflow-hidden py-32 text-center flex flex-col gap-6">
  <div class="mx-auto bg-card rounded-lg p-1">
    <VaultIcon size="64" class="mx-auto text-primary" />
  </div>
  <div>
    <h1 class="mb-6 text-pretty text-2xl font-bold tracking-tight lg:text-5xl">
      Unbreakable Security,<br /> Forged in Iron
    </h1>
    <p class="text-muted-foreground mx-auto max-w-3xl lg:text-xl">
      Ferrum Vault is a next-generation password manager built in Rust,<br />
      engineered for speed, reliability, and uncompromising safety.
    </p>
  </div>
  <div class="mt-6 flex justify-center gap-3">
    <Button class="" href="/register">Get Started</Button>
    <Button variant="outline" class="group" href="/about">
      Learn more <LinkIcon class="inline" />
    </Button>
  </div>
</section>
