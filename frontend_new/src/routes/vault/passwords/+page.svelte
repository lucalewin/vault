<script lang="ts">
  import * as Empty from "$lib/components/ui/empty/index.js";
  import { Button } from "$lib/components/ui/button/index.js";
  import ArrowUpRightIcon from "@lucide/svelte/icons/arrow-up-right";
  import KeyRoundIcon from "@lucide/svelte/icons/key-round";
  import CreateModal from "./create-modal.svelte";

  import { vault } from "$lib/stores/credentialStore";
  import { get } from "svelte/store";
  import * as Card from "$lib/components/ui/card";
  import DataTable from "./data-table.svelte";
  import { columns } from "./columns";
  import { Input } from "$lib/components/ui/input";
  import { credentials } from "$lib/state";
  import { onMount } from "svelte";
  import { token } from "$lib/stores/authStore";
  import { fetchCredentials } from "$lib";

  let empty = $derived(credentials.current.length === 0);

  let sorted = $derived(
    credentials.current.toSorted((a, b) => a.service.localeCompare(b.service)),
  );

  onMount(async () => {
    await fetchCredentials();
  });
</script>

{#if !empty}
  <Card.Root class="max-w-4xl w-full mx-auto">
    <Card.Header>
      <Card.Title>Your Passwords</Card.Title>
      <Card.Description></Card.Description>
    </Card.Header>
    <Card.Content>
      <div class="flex flex-col-reverse md:flex-row justify-between gap-4 my-2">
        <Input placeholder="Search Passwords" class="max-w-sm" />
        <div class="flex gap-2">
          <Button variant="outline" disabled>Import Password</Button>
          <CreateModal />
        </div>
      </div>
      <DataTable data={sorted} {columns} />
    </Card.Content>
  </Card.Root>
{:else}
  <Empty.Root>
    <Empty.Header>
      <Empty.Media variant="icon">
        <KeyRoundIcon />
      </Empty.Media>
      <Empty.Title>No Passwords Yet</Empty.Title>
      <Empty.Description>
        You haven't created any passwords yet. Get started by creating your
        first password.
      </Empty.Description>
    </Empty.Header>
    <Empty.Content>
      <div class="flex gap-2">
        <!-- <Button>Create Password</Button> -->
        <CreateModal />
        <Button variant="outline">Import Password</Button>
      </div>
    </Empty.Content>
    <Button variant="link" class="text-muted-foreground" size="sm">
      <a href="#/">
        Learn More <ArrowUpRightIcon class="inline" />
      </a>
    </Button>
  </Empty.Root>
{/if}
