import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import { VitePWA } from "vite-plugin-pwa";

export default defineConfig({
  plugins: [
    react(),
    VitePWA({
      registerType: "autoUpdate",
      manifest: {
        name: "知了发布",
        short_name: "知了",
        description: "校园公告发布端",
        theme_color: "#1890ff",
        display: "standalone",
      },
    }),
  ],
  server: {
    port: 5174,
  },
});
