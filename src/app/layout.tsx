import type { Metadata } from "next";
import "./globals.css";

import { font_sans, font_mono } from "@/styles/font";

export const metadata: Metadata = {
  title: "Stylus by Example",
  description:
    "An introduction to Arbitrum Stylus with simple code examples in Rust and WASM",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html className={`${font_mono.variable} ${font_sans.variable}`}>
      <body>{children}</body>
    </html>
  );
}
