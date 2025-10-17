import { get } from "svelte/store";
import { token } from "./stores/authStore";
import { authenticator, credentials } from "./state";

export type Authenticator = {
  id: string;
  service: string;
  username: string;
  code: string;
};

export function isValidDomain(str: string) {
  // Regex to check for common domain name format:
  // ^: start of string
  // (?:[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?\.)+: one or more domain labels (parts separated by dots)
  //   - [a-z0-9]: must start with a letter/number
  //   - (?:[a-z0-9-]{0,61}[a-z0-9])?: optional internal part (up to 61 chars of letters/numbers/hyphens, ending with a letter/number)
  //   - \.: followed by a dot (for intermediate labels)
  // [a-z0-9][a-z0-9-]{0,61}[a-z0-9]: the final TLD (must be at least 2 chars, like 'com', 'co.uk')
  // $: end of string
  const domainRegex = new RegExp(
    /^((?:[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?\.)+)(?:[a-z0-9][a-z0-9-]{0,61}[a-z0-9])$/i,
  );

  // Check the overall length (max 253 characters) and the regex pattern
  return str.length <= 253 && domainRegex.test(str);
}

export async function fetchCredentials() {
  try {
    const resp = await fetch("/api/v1/credentials", {
      method: "GET",
      headers: {
        Authorization: `Bearer ${get(token)}`,
      },
    });
    const data = await resp.json();
    credentials.current = data.credentials;
  } catch (error) {
    console.error(error);
  }
}

export async function fetchAuthenticator() {
  try {
    const resp = await fetch("/api/v1/authenticator", {
      method: "GET",
      headers: {
        Authorization: `Bearer ${get(token)}`,
      },
    });
    const data = await resp.json();
    console.log(data);
    authenticator.current = data;
  } catch (error) {
    console.error(error);
  }
}
