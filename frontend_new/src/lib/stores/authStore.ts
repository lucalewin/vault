import { writable } from "svelte/store";
import { browser } from "$app/environment";

const TOKEN_KEY = "jwt_token";

const initialToken = browser ? localStorage.getItem(TOKEN_KEY) : null;

export const token = writable(initialToken);

if (browser) {
  token.subscribe((token) => {
    if (token) {
      localStorage.setItem(TOKEN_KEY, token);
    } else {
      localStorage.removeItem(TOKEN_KEY);
    }
  });
}

export function logout() {
  token.set(null);
}
