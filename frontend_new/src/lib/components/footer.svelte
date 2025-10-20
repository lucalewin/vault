<script lang="ts">
  import GithubIcon from "@lucide/svelte/icons/github";
</script>

<footer class="py-12 border-t">
  <div class="max-w-6xl mx-auto px-8 grid grid-cols-3 gap-8">
    <!-- Left: Brand -->
    <div>
      <h3 class="text-2xl font-bold text-white">Ferrum Vault</h3>
      <p class="mt-4 text-gray-400 text-sm">
        Password security, forged in Rust.<br />
        Fast, reliable, and uncompromisingly safe.
      </p>
    </div>

    <!-- Middle: Links -->
    <div class="flex flex-col space-y-3 text-sm">
      <a href="/features" class="hover:text-white transition">Features</a>
      <a href="/pricing" class="hover:text-white transition">Pricing</a>
      <a href="/docs" class="hover:text-white transition">Docs</a>
      <a href="/support" class="hover:text-white transition">Support</a>
    </div>

    <!-- Right: Social & Legal -->
    <div class="flex flex-col items-end justify-between">
      <!-- Social icons -->
      <div class="flex space-x-4 mb-4">
        <a
          href="https://github.com/lucalewin/vault"
          class="hover:text-primary bg-muted rounded-full p-2 flex items-center justify-center"
        >
          <GithubIcon />
        </a>
      </div>

      <!-- Copyright -->
      <p class="text-muted-foreground text-xs">
        &copy; 2025 Luca Lewin. All rights reserved.
      </p>
    </div>
  </div>
</footer>
