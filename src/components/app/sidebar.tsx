import { BasicExamples } from "@/data/sidebar";
import { css } from "@/styled-system/css";
import { stack } from "@/styled-system/patterns";
import Link from "next/link";
import { Stack } from "@/styled-system/jsx";
import { Button } from "@/components/ui/button";
interface SidebarProps extends React.HtmlHTMLAttributes<HTMLDivElement> {
  basicExamples: BasicExamples[];
}

import { SidebarLinks } from "@/components/app/sidebar_links";

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
          fontSize: { base: "20px", "2xl": "20px" },
          fontWeight: "bold",
          mb: "1",
          px: "4",
        })}
      >
        Basic
      </h2>
      <Stack gap={1}>
        <SidebarLinks routes={basicExamples} />
      </Stack>
    </div>
  );
}
