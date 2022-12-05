import { defineConfig } from "vite";
import { resolve } from "path";

import Inspect from "vite-plugin-inspect";
import SignalJSX from "@westhide/vite-plugin-signal-jsx";

export default defineConfig({
  resolve: {
    alias: {
      "@": `${resolve(__dirname, "src")}/`,
    },
  },
  plugins: [Inspect(), SignalJSX()],
});
