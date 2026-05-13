import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import path from "path";

export default defineConfig(({ mode }) => ({
  plugins: [react()],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    host: "127.0.0.1",
    watch: {
      ignored: ["**/src-tauri/**", "**/target/**"],
    },
  },
  envPrefix: ["VITE_", "TAURI_"],
  build: {
    // WebView2 on Windows supports all modern JS — target Chrome 105+.
    target: "chrome105",
    minify: "esbuild",
    sourcemap: false,
    // Don't inline anything; let the browser cache assets separately.
    assetsInlineLimit: 0,
    rollupOptions: {
      output: {
        // Vendor split keeps the main chunk small and improves caching:
        // React core rarely changes, so it stays cached across releases.
        manualChunks: {
          "vendor-react": ["react", "react-dom"],
          "vendor-router": ["react-router-dom"],
          "vendor-query": ["@tanstack/react-query"],
          "vendor-tauri": [
            "@tauri-apps/api",
            "@tauri-apps/plugin-updater",
            "@tauri-apps/plugin-process",
            "@tauri-apps/plugin-autostart",
          ],
        },
      },
    },
    chunkSizeWarningLimit: 600,
    // esbuild drop options for production
    ...(mode === "production" && {
      esbuildOptions: {
        drop: ["console", "debugger"],
      },
    }),
  },
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
    },
  },
}));
