import { PersistedState } from "runed";
import type { PwCredential } from "../routes/vault/passwords/columns";
import type { Authenticator } from "$lib";

export const credentials = new PersistedState(
  "credentials",
  [] as PwCredential[],
  {
    storage: "session",
  },
);

export const authenticator = new PersistedState(
  "authenticator",
  [] as Authenticator[],
  {
    storage: "session",
  },
);
