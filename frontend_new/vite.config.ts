import tailwindcss from "@tailwindcss/vite";
import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [tailwindcss(), sveltekit()],
  server: {
    proxy: {
      "/api": {
        // target: "http://localhost:8000",
        target: "https://vault.lucalewin.dev",
        changeOrigin: true,
        ws: true,
        secure: true,
      },
    },
  },
});
