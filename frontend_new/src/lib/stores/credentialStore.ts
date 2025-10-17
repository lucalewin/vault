import { writable } from "svelte/store";
import { browser } from "$app/environment";

const VAULT_KEY = "vault";

const initialVault = browser
  ? JSON.parse(localStorage.getItem(VAULT_KEY) ?? "{}")
  : null;

export const vault = writable(initialVault);

if (browser) {
  vault.subscribe((vault) => {
    if (vault) {
      localStorage.setItem(VAULT_KEY, JSON.stringify(vault));
    } else {
      localStorage.removeItem(VAULT_KEY);
    }
  });
}

export function logout() {
  vault.set(null);
}
