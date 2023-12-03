"use client";

import * as React from "react";
import { MoonIcon, SunIcon } from "@radix-ui/react-icons";
import { useTheme } from "next-themes";

import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown_menu";
import { css } from "@/styled-system/css";

export function ModeToggle() {
  const { setTheme } = useTheme();

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button
          variant="link"
          className={css({
            w: "36px",
            h: "36px",
            px: "0",
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
          <SunIcon
            className={css({
              h: "24px",
              w: "24px",
              display: "inline",
              _dark: { display: "none" },
            })}
          />
          <MoonIcon
            className={css({
              h: "24px",
              w: "24px",
              display: "none",
              _dark: { display: "inline" },
            })}
          />
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent align="end">
        <DropdownMenuItem onClick={() => setTheme("light")}>
          Light
        </DropdownMenuItem>
        <DropdownMenuItem onClick={() => setTheme("dark")}>
          Dark
        </DropdownMenuItem>
        <DropdownMenuItem onClick={() => setTheme("system")}>
          System
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
