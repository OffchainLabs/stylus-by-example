import Link from "next/link";
import Image from "next/image";

import { ThemeProvider } from "@/app/theme_provider";

import { HamburgerMenuIcon } from "@radix-ui/react-icons";
import { font_sans, font_mono } from "@/styles/font";
import { Sidebar } from "@/components/app/sidebar";
import { basicExamples } from "@/data/sidebar";
import { HStack, Stack } from "@/styled-system/jsx";
import { css } from "@/styled-system/css";
import { flex, hstack, stack } from "@/styled-system/patterns";
import { ModeToggle } from "@/components/app/mode_toggle";
import {
  NavigationMenu,
  NavigationMenuContent,
  NavigationMenuIndicator,
  NavigationMenuItem,
  NavigationMenuLink,
  NavigationMenuList,
  NavigationMenuTrigger,
  NavigationMenuViewport,
} from "@/components/ui/navigation_menu";
import {
  Sheet,
  SheetClose,
  SheetContent,
  SheetDescription,
  SheetHeader,
  SheetTitle,
  SheetTrigger,
} from "@/components/ui/sheet";
import { Button } from "@/components/ui/button";

const SIDEBAR_WIDTH = "256px";

export default function Navigation() {
  return (
    <div
      className={hstack({
        borderBottomWidth: { base: "0", md: "1px" },
        borderTopWidth: { base: "1px", md: "0" },
        py: "8px",
        shadow: "lg",
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
            fontSize: { base: "md", md: "xl" },
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
      <div
        className={css({ flex: "1", display: { base: "none", md: "block" } })}
      >
        <NavigationMenu>
          <NavigationMenuList>
            <NavigationMenuItem>
              <NavigationMenuLink asChild>
                <Link
                  href="/"
                  className={css({
                    display: "flex",
                    h: "full",
                    w: "full",
                    userSelect: "none",
                    flexDirection: "column",
                    justifyContent: "center",
                    rounded: "md",
                    bgGradient: "to-b",
                    gradientFromAlpha: "muted/50",
                    gradientToAlpha: "muted",
                    p: "2",
                    textDecoration: "none",
                    outline: "none",
                    _focus: {
                      shadow: "md",
                    },
                  })}
                >
                  Linnnk
                </Link>
              </NavigationMenuLink>
            </NavigationMenuItem>
          </NavigationMenuList>
        </NavigationMenu>
      </div>

      <div
        className={hstack({
          pr: "8px",
          justify: "flex-end",
          bg: "pink.100",
          flex: { base: "1", md: "0" },
        })}
      >
        <ModeToggle />

        <Sheet>
          <SheetTrigger
            className={flex({
              w: "36px",
              h: "36px",
              px: "0",
              userSelect: "none",
              align: "center",
              justifyContent: "center",
              rounded: "md",
              bgGradient: "to-b",
              gradientFromAlpha: "muted/50",
              gradientToAlpha: "muted",
              p: "2",
              textDecoration: "none",
              outline: "none",
              cursor: "pointer",
              display: { base: "flex", md: "none" },
              _focus: {
                shadow: "md",
              },
            })}
          >
            <HamburgerMenuIcon
              className={css({
                h: "24px",
                w: "24px",
              })}
            />
          </SheetTrigger>
          <SheetContent className={css({ p: "0" })}>
            <div
              className={stack({
                pt: "4",
                px: "2",
                pb: "24",
                gap: "0",
              })}
            >
              <h2
                className={css({
                  fontSize: "lg",
                  fontWeight: "bold",
                  mb: "1",
                  px: "4",
                })}
              >
                Basic
              </h2>
              <Stack gap={1}>
                {basicExamples?.map(({ route, title }, i) => (
                  <Button
                    asChild
                    key={`${title}-${i}`}
                    variant="link"
                    size="lg"
                    className={css({
                      w: "full",
                      justifyContent: "flex-start",
                      fontWeight: "normal",
                      h: "32px",
                      paddingInline: "16px",
                    })}
                  >
                    <SheetClose asChild>
                      <Link href={route}>{title}</Link>
                    </SheetClose>
                  </Button>
                ))}
              </Stack>
            </div>
          </SheetContent>
        </Sheet>
      </div>
    </div>
  );
}
