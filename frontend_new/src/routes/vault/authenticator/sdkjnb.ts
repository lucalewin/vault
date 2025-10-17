// import type { PageLoad } from "./$types";
// import { token } from "$lib/stores/authStore";
// import { get } from "svelte/store";
// import { vault } from "$lib/stores/credentialStore";

// export const load: PageLoad = async ({ fetch }) => {
//   try {
//     const resp = await fetch("/api/v1/authenticator", {
//       method: "GET",
//       headers: {
//         Authorization: `Bearer ${get(token)}`,
//       },
//     });
//     const data = await resp.json();
//     vault.set(data);
//     console.log(data);
//   } catch (error) {
//     console.error(error);
//   }

//   return {};
// };
