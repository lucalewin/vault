<script lang="ts">
  import ShieldIcon from "@lucide/svelte/icons/shield";
  import SpeedIcon from "@lucide/svelte/icons/gauge";
  import CloudCheckIcon from "@lucide/svelte/icons/cloud-check";
  import { Button } from "./ui/button";

  let features = [
    {
      icon: ShieldIcon,
      title: "Ironclad Security",
      description:
        "End-to-end encryption ensures your passwords stay private - even from us",
    },
    {
      icon: SpeedIcon,
      title: "Blazing Speed",
      description:
        "Rust delivers unmatched speed and reliability - making vault access instant.",
    },
    {
      icon: CloudCheckIcon,
      title: "Sync Across Devices",
      description: "Access your vault from anywhere, anytime",
    },
  ];
</script>

<section class="py-32">
  <div class="container mx-auto px-8">
    <div class="text-center mb-16">
      <h2 class="text-4xl font-bold">
        Why Choose <span class="text-primary italic">Ferrum Vault</span>?
      </h2>
      <p class="mt-4 text-lg text-muted-foreground">
        Built with Rust, Ferrum Vault combines rock-solid security with
        blazing-fast performance.
      </p>
    </div>

    <div class="grid grid-cols-3 gap-12">
      {#each features as feature}
        {@const Icon = feature.icon}
        <div class="text-center bg-card p-8 rounded-lg">
          <div
            class="mx-auto flex items-center justify-center h-16 w-16 rounded-2xl bg-black text-primary mb-6"
          >
            <Icon size={32} />
          </div>
          <h3 class="text-xl font-semibold">{feature.title}</h3>
          <p class="mt-3 text-muted-foreground">
            {feature.description}
          </p>
        </div>
      {/each}
    </div>

    <div class="mt-18 text-center">
      <Button href="/features">More Features</Button>
    </div>
  </div>
</section>
