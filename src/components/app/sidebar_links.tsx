"use client";
import Link from "next/link";
import { useSelectedLayoutSegments } from "next/navigation";
import { Button } from "@/components/ui/button";
import { RouteInfo } from "@/data/routes";
import { css } from "@/styled-system/css";

interface SidebarLinksProps {
  routes: RouteInfo[];
}

const TEXT_COLOR = {
  _hover: "pink.400",
};
const ACTIVE_ROUTE_COLOR = { _dark: "white", _light: "black" };

export function SidebarLinks({ routes }: SidebarLinksProps) {
  const segments = useSelectedLayoutSegments();
  const ACTIVE_ROUTE = `/${segments.join("/")}`;

  return (
    <>
      {routes?.map(({ route, title }, i) => (
        <Button
          asChild
          key={`${title}-${i}`}
          variant="link"
          className={css({
            w: "full",
            justifyContent: "flex-start",
            fontWeight: "normal",
            fontSize: { base: "16px", "2xl": "18px" },
            paddingInline: "16px",
            h: { base: "32px", "2xl": "36px" },
            color: route === ACTIVE_ROUTE ? ACTIVE_ROUTE_COLOR : TEXT_COLOR,
            textDecoration: route === ACTIVE_ROUTE ? "underline" : "",
          })}
        >
          <Link href={route}>{title}</Link>
        </Button>
      ))}
    </>
  );
}
