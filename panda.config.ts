import { defineConfig, defineGlobalStyles } from "@pandacss/dev";

export default defineConfig({
  presets: ["@shadow-panda/preset"],

  // Whether to use css reset
  preflight: true,

  // Where to look for your css declarations
  include: [
    "mdx-components.tsx",
    "./src/**/*.{js,jsx,ts,tsx}",
    "./pages/**/*.{js,jsx,ts,tsx}",
  ],

  // Files to exclude
  exclude: [],

  // Useful for theme customization
  theme: {
    extend: {
      tokens: {
        fonts: {
          sans: { value: "var(--font-sans), sans-serif" },
          mono: { value: "var(--font-mono), monospace" },
          body: { value: "var(--font-body), sans-serif" },
          heading: { value: "var(--font-heading), sans-serif" },
        },
      },
    },
  },

  jsxFramework: "react",

  // The output directory for your css system
  outdir: "./src/styled-system",
});
