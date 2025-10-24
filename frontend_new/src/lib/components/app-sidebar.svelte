<script lang="ts">
  import LayoutDashboardIcon from "@lucide/svelte/icons/layout-dashboard";
  import LockIcon from "@lucide/svelte/icons/lock";
  import SettingsIcon from "@lucide/svelte/icons/settings";
  import LogOutIcon from "@lucide/svelte/icons/log-out";
  import IdCardIcon from "@lucide/svelte/icons/id-card";
  import KeyRoundIcon from "@lucide/svelte/icons/key-round";
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import VaultSwitcher from "./vault-switcher.svelte";
  import { page } from "$app/state";
  import { token } from "$lib/stores/authStore";
  import { goto } from "$app/navigation";

  // Menu items.
  const items = [
    {
      title: "Overview",
      url: "/vault",
      icon: LayoutDashboardIcon,
    },
    {
      title: "Passwords",
      url: "/vault/passwords",
      icon: KeyRoundIcon,
    },
    {
      title: "Authenticator",
      url: "/vault/authenticator",
      icon: LockIcon,
    },
    {
      title: "Identities",
      url: "/vault/identities",
      icon: IdCardIcon,
    },
  ];

  const logout = async () => {
    token.set(null);
    await goto("/");
  };
</script>

<Sidebar.Root collapsible="icon" variant="sidebar">
  <Sidebar.Header>
    <VaultSwitcher versions={["Personal", "Work"]} defaultVersion="Personal" />
  </Sidebar.Header>
  <Sidebar.Content>
    <Sidebar.Group>
      <Sidebar.GroupLabel>Application</Sidebar.GroupLabel>
      <Sidebar.GroupContent>
        <Sidebar.Menu>
          {#each items as item (item.title)}
            <Sidebar.MenuItem>
              <Sidebar.MenuButton isActive={item.url === page.url.pathname}>
                {#snippet child({ props })}
                  <a href={item.url} {...props}>
                    <item.icon />
                    <span>{item.title}</span>
                  </a>
                {/snippet}
              </Sidebar.MenuButton>
            </Sidebar.MenuItem>
          {/each}
        </Sidebar.Menu>
      </Sidebar.GroupContent>
    </Sidebar.Group>
  </Sidebar.Content>
  <Sidebar.Footer>
    <Sidebar.Menu>
      <Sidebar.MenuItem>
        <Sidebar.MenuButton onclick={() => logout()}>
          <LogOutIcon />
          <span>Logout</span>
        </Sidebar.MenuButton>
      </Sidebar.MenuItem>
    </Sidebar.Menu>
    <Sidebar.Menu>
      <Sidebar.MenuItem>
        <Sidebar.MenuButton>
          <SettingsIcon />
          <span>Settings</span>
        </Sidebar.MenuButton>
      </Sidebar.MenuItem>
    </Sidebar.Menu>
  </Sidebar.Footer>
</Sidebar.Root>
