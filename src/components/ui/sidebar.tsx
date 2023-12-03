import { BasicExamples } from "@/data/sidebar";
import { css } from "@/styled-system/css";
import { stack } from "@/styled-system/patterns";
import Link from "next/link";
import { Stack } from "@/styled-system/jsx";
import { Button } from "@/components/ui/button";

interface SidebarProps extends React.HtmlHTMLAttributes<HTMLDivElement> {
  basicExamples: BasicExamples[];
}

export function Sidebar({ basicExamples }: SidebarProps) {
  return (
    <div className={stack({ background: "green.300" })}>
      <h2
        className={css({
          fontSize: "lg",
          fontWeight: "bold",
          mb: "2",
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
            variant="ghost"
            className={css({
              w: "full",
              justifyContent: "flex-start",
              fontWeight: "normal",
            })}
          >
            <Link href={route}>{title}</Link>
          </Button>
        ))}
      </Stack>
    </div>
  );
}
