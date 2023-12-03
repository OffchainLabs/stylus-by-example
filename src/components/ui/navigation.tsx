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
import { css } from "@/styled-system/css";
import Link from "next/link";

export default function Navigation() {
  return (
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
  );
}
