import type { Metadata } from "next";

import { HStack, Stack } from "@/styled-system/jsx";
import { css, cx } from "@/styled-system/css";
import { container, flex, stack } from "@/styled-system/patterns";
import Navigation from "@/components/app/navigation";
import { font_mono, font_body, font_heading } from "@/styles/font";

import "./globals.css";
import { ThemeProvider } from "./theme_provider";
import SideNav from "./side_nav";
import { Breadcrumbs } from "@/components/app/breadcrumbs";
import { PageScroll } from "@/components/app/page_scroll";
import { PHProvider } from "./providers";
import dynamic from "next/dynamic";

const PostHogPageView = dynamic(() => import("./PostHogPageView"), {
  ssr: false,
});

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
    <html
      suppressHydrationWarning
      className={`${font_mono.variable} ${font_body.variable} ${font_heading.variable}`}
    >
      <PHProvider>
        <body
          className={css({
            overflow: "hidden",
            h: "100dvh",
            w: "100dvw",
            fontFamily: "body",
          })}
        >
          <PostHogPageView />

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
              <HStack
                className={css({
                  flexGrow: "1",
                  overflow: "hidden",
                })}
              >
                <SideNav />
                <PageScroll>
                  <div
                    className={container({
                      maxW: "5xl",
                    })}
                  >
                    <Breadcrumbs />
                    {children}
                  </div>
                </PageScroll>
              </HStack>
            </Stack>
          </ThemeProvider>
        </body>
      </PHProvider>
    </html>
  );
}
