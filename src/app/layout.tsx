import type { Metadata } from "next";
import Image from "next/image";

import "./globals.css";
import { ThemeProvider } from "./theme_provider";

import { font_sans, font_mono } from "@/styles/font";
import { Sidebar } from "@/components/ui/sidebar";
import { basicExamples } from "@/data/sidebar";
import { HStack, Stack } from "@/styled-system/jsx";
import { css } from "@/styled-system/css";
import { flex, hstack } from "@/styled-system/patterns";
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
          <Stack gap="0">
            <div
              className={hstack({
                borderBottomStyle: "solid",
                borderBottomWidth: "1px",
                borderColor: "gray.600",
                py: "8px",
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
            <HStack className={css({ minH: "100vh", bg: "pink.100" })}>
              <Stack
                className={css({
                  bg: "red.300",
                  minH: "100vh",
                  py: "2",
                  px: "3",
                  pb: "12",
                  w: "256px",
                })}
              >
                <Sidebar basicExamples={basicExamples} />
              </Stack>
              <div
                className={flex({
                  grow: "1",
                  bg: "blue.100",
                  minH: "100vh",
                  pt: "6",
                  pb: "12",
                  px: "4",
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
