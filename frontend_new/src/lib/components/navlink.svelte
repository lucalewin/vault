<script lang="ts">
  import { Button } from "./ui/button";

  let { href, title, ...props } = $props();
</script>

<!-- <a class="py-2 px-4 rounded-lg hover:bg-muted" {href} {...props}>{title}</a> -->
<Button {href} variant="ghost" class="text-base">{title}</Button>
