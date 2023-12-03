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
            <Link href={route}>{title}</Link>
          </Button>
        ))}
      </Stack>
    </div>
  );
}
