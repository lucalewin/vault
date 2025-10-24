<!--
	Installed from @ieedan/shadcn-svelte-extras
-->

<script lang="ts">
  import { CopyButton } from "$lib/components/ui/copy-button";
  import type { PasswordCopyButtonProps } from "./types.js";
  import { cn } from "$lib/utils/utils.js";
  import {
    usePassword,
    usePasswordCopy,
    usePasswordInput,
  } from "./password.svelte.js";
  import Button from "../button/button.svelte";
  import RefreshIcon from "@lucide/svelte/icons/refresh-cw";

  let {
    ref = $bindable(null),
    class: className,
    text = $bindable(""),
    ...rest
  }: { ref: any; class: string; text: string } = $props();

  const generatePassword = () => {
    const upper = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const lower = "abcdefghijklmnopqrstuvwxyz";
    const numbers = "0123456789";
    const symbols = "!@#$%^&*()_+[]{}|;:,.<>?";
    const allChars = upper + lower + numbers + symbols;

    let generatedPassword = "";
    generatedPassword += upper.charAt(Math.floor(Math.random() * upper.length));
    generatedPassword += lower.charAt(Math.floor(Math.random() * lower.length));
    generatedPassword += numbers.charAt(
      Math.floor(Math.random() * numbers.length),
    );
    generatedPassword += symbols.charAt(
      Math.floor(Math.random() * symbols.length),
    );

    for (let i = 4; i < 25; i++) {
      generatedPassword += allChars.charAt(
        Math.floor(Math.random() * allChars.length),
      );
    }

    return generatedPassword
      .split("")
      .sort(() => 0.5 - Math.random())
      .join("");
  };

  const state = usePasswordCopy();
</script>

<Button
  variant="ghost"
  onclick={() => {
    text = generatePassword();
    // state.root.passwordState.value = generatePassword();
    // console.log("Password generated:", state.root.passwordState.value);
  }}
  class={cn(
    "text-muted-foreground absolute top-1/2 right-0 size-9 min-w-0 -translate-y-1/2 hover:!bg-transparent",
    className,
  )}
>
  <!-- <div in:scale={{ duration: animationDuration, start: 0.85 }}> -->
  <div>
    <RefreshIcon tabindex={-1} />
    <span class="sr-only">Generate</span>
  </div>
</Button>

<!-- <CopyButton
  {...rest}
  bind:ref
  text={state.root.passwordState.value}
  tabindex={-1}
  class={cn(
    "text-muted-foreground absolute top-1/2 right-0 size-9 min-w-0 -translate-y-1/2 hover:!bg-transparent",
    className,
  )}
/> -->
