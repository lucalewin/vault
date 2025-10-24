<script lang="ts">
  import * as Empty from "$lib/components/ui/empty/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import ArrowUpRightIcon from "@lucide/svelte/icons/arrow-up-right";
  import IdCardIcon from "@lucide/svelte/icons/id-card";
</script>

<Empty.Root class="flex justify-center items-center">
  <Empty.Header>
    <Empty.Media variant="icon">
      <IdCardIcon />
    </Empty.Media>
    <Empty.Title>No Identities Yet</Empty.Title>
    <Empty.Description>
      You haven't created any identities yet. Get started by creating your first
      identity.
    </Empty.Description>
  </Empty.Header>
  <Empty.Content>
    <div class="flex gap-2">
      <Button>Create Identity</Button>
      <Button variant="outline" disabled>Import Identity</Button>
    </div>
  </Empty.Content>
  <Button variant="link" class="text-muted-foreground" size="sm">
    <a href="/docs/identities">
      Learn More <ArrowUpRightIcon class="inline" />
    </a>
  </Button>
</Empty.Root>
