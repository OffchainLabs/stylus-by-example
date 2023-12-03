import type { Metadata } from "next";
import Image from "next/image";

import "./globals.css";
import { ThemeProvider } from "./theme_provider";

import { font_sans, font_mono } from "@/styles/font";
import { Sidebar } from "@/components/ui/sidebar";
import { basicExamples } from "@/data/sidebar";
import { HStack, Stack } from "@/styled-system/jsx";
import { css } from "@/styled-system/css";
import { box, flex, hstack } from "@/styled-system/patterns";
import Navigation from "@/components/ui/navigation";
import { ModeToggle } from "@/components/app/mode_toggle";

export const metadata: Metadata = {
  title: "Stylus by Example",
  description:
    "An introduction to Arbitrum Stylus with simple code examples in Rust and WASM",
};

const SIDEBAR_WIDTH = "256px";

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html className={`${font_mono.variable} ${font_sans.variable}`}>
      <body>
        <ThemeProvider attribute="class" defaultTheme="system" enableSystem>
          <Stack
            gap="0"
            h="100vh"
            overflow="hidden"
            bg="stone.100"
            _dark={{ bg: "stone.800" }}
          >
            <div
              className={hstack({
                borderBottomStyle: "solid",
                borderBottomWidth: "1px",
                py: "8px",
                shadow: "lg",
                position: "sticky",
                top: "0",
                bg: "stone.100",
                borderColor: "stone.300",
                _dark: { bg: "stone.950", borderColor: "stone.900" },
              })}
            >
              <div
                className={hstack({
                  w: SIDEBAR_WIDTH,
                  px: "8px",
                })}
              >
                <Image
                  alt="Arbitrum Stylus logo"
                  width={32}
                  height={32}
                  src="/stylus_logo_mark.svg"
                />
                <h2
                  className={css({
                    fontSize: "xl",
                    fontFamily: "sans",
                    fontWeight: "bold",
                    letterSpacing: "tighter",
                    ml: "-1",
                    mr: "6",
                  })}
                >
                  Stylus by Example
                </h2>
              </div>
              <div className={css({ flex: "1" })}>
                <Navigation />
              </div>
              <div className={css({ pr: "8px" })}>
                <ModeToggle />
              </div>
            </div>
            <HStack className={css({ h: "full" })}>
              <Stack
                className={css({
                  w: "256px",
                  h: "full",
                  overflowY: "auto",
                  borderRightWidth: "1px",
                  borderColor: "stone.300",
                  background: "stone.200",
                  _dark: {
                    borderColor: "stone.800",
                    background: "stone.900",
                  },
                })}
              >
                <Sidebar basicExamples={basicExamples} />
              </Stack>
              <div
                className={flex({
                  grow: "1",
                  pt: "6",
                  pb: "12",
                  px: "4",
                  h: "full",
                })}
              >
                {children}
              </div>
            </HStack>
          </Stack>
        </ThemeProvider>
      </body>
    </html>
  );
}
