import { BasicExamples, gettingStarted } from "@/data/routes";
import { css } from "@/styled-system/css";
import { stack } from "@/styled-system/patterns";
import { Stack } from "@/styled-system/jsx";

interface SidebarProps extends React.HtmlHTMLAttributes<HTMLDivElement> {
  basicExamples: BasicExamples[];
}

import { SidebarLinks } from "@/components/app/sidebar_links";
import { ReactNode } from "react";

function SidebarHeading({ children }: { children: ReactNode }) {
  return (
    <h2
      className={css({
        fontSize: { base: "20px", md: "22px", "2xl": "22px" },
        fontWeight: "bold",
        mb: "1",
        px: "4",
        fontFamily: "heading",
      })}
    >
      {children}
    </h2>
  );
}

export function Sidebar({ basicExamples }: SidebarProps) {
  return (
    <div
      className={stack({
        pt: "4",
        px: "1",
        pb: "24",
        gap: "0",
      })}
    >
      <SidebarHeading>Getting Started</SidebarHeading>
      <Stack gap={1} mb={4}>
        <SidebarLinks routes={gettingStarted} />
      </Stack>
      <SidebarHeading>Basic</SidebarHeading>
      <Stack gap={1} mb={4}>
        <SidebarLinks routes={basicExamples} />
      </Stack>
    </div>
  );
}
