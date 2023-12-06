import type { Metadata } from "next";

import { HStack, Stack } from "@/styled-system/jsx";
import { css } from "@/styled-system/css";
import { container, flex, stack } from "@/styled-system/patterns";
import Navigation from "@/components/app/navigation";
import { font_mono, font_sans } from "@/styles/font";

import "./globals.css";
import { ThemeProvider } from "./theme_provider";
import SideNav from "./side_nav";

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
      <body
        className={css({
          overflow: "hidden",
          h: "100dvh",
          w: "100dvw",
        })}
      >
        <ThemeProvider attribute="class" defaultTheme="system" enableSystem>
          <Stack
            gap="0"
            h="full"
            w="full"
            overflow="hidden"
            bg="stone.50"
            flexDir={{ base: "column-reverse", md: "column" }}
            _dark={{ bg: "stone.800" }}
          >
            <Navigation />
            <HStack className={css({ flexGrow: "1", overflow: "hidden" })}>
              <SideNav />
              <div
                className={css({
                  flexGrow: "1",
                  h: "full",
                  overflowY: "auto",
                  pt: "6",
                  pb: "12",
                  px: "4",
                })}
              >
                <div
                  className={container({
                    maxW: { base: "4xl", "2xl": "8xl" },
                  })}
                >
                  {children}
                </div>
              </div>
            </HStack>
          </Stack>
        </ThemeProvider>
      </body>
    </html>
  );
}
