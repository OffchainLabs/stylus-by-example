import { defineConfig } from "@pandacss/dev";

export default defineConfig({
  presets: ["@shadow-panda/preset"],

  // Whether to use css reset
  preflight: true,

  // Where to look for your css declarations
  include: ["./src/**/*.{js,jsx,ts,tsx}", "./pages/**/*.{js,jsx,ts,tsx}"],

  // Files to exclude
  exclude: [],

  // Useful for theme customization
  theme: {
    extend: {
      tokens: {
        fonts: {
          sans: { value: "var(--font-sans), sans-serif" },
          mono: { value: "var(--font-mono), monospace" },
        },
      },
    },
  },

  jsxFramework: "react",

  // The output directory for your css system
  outdir: "./src/styled-system",
});
